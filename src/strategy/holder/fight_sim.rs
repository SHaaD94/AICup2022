use crate::debug_interface::DebugInterface;
use crate::debugging::{TRANSPARENT_BLACK, TRANSPARENT_GREEN, TRANSPARENT_ORANGE};
use crate::model::{Obstacle, Unit, Vec2, WeaponProperties};
use crate::strategy::behaviour::behaviour::{my_units_magnet_score, zone_penalty};
use crate::strategy::holder::fight_sim::FightSimResult::{DRAW, LOST, WON};
use crate::strategy::holder::{get_all_enemy_units, get_constants, get_game};
use crate::strategy::util::{intersects_with_obstacles_vec, rotate};
use itertools::{all, Itertools};
use std::cmp::max;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

const GROUP_RADIUS: f64 = 15.0;

#[derive(Clone, Debug)]
pub enum FightSimResult {
    // number of people died
    WON(i32),
    LOST,
    DRAW,
}

#[derive(Clone, Debug)]
pub struct FightSim {
    pub result: FightSimResult,
    pub allies: HashSet<i32>,
    pub enemies: HashSet<i32>,
}

impl FightSim {
    pub fn enemy_units(&self) -> Vec<&Unit> {
        get_all_enemy_units()
            .iter()
            .filter(|e| self.enemies.contains(&e.id))
            .collect_vec()
    }
}

pub fn create_fight_simulations(
    debug_interface: &mut Option<&mut DebugInterface>,
) -> Vec<FightSim> {
    let my_groups = get_game()
        .my_units()
        .iter()
        .map(|unit| {
            let mut a = get_my_other_units_nearby(unit);
            a.push(unit);
            a
        })
        .unique_by(|a| a.iter().map(|e| e.id.to_string()).sorted().join(","))
        .collect_vec();
    // println!("calculating sim for groups: {}", my_groups.iter().map(|e| e.len().to_string()).join(", "));

    let enemy_groups = find_enemy_groups();

    let mut sims = Vec::new();
    for my_group in &my_groups {
        for enemy_group in &enemy_groups {
            let sim_res = simulation(&my_group, &enemy_group);
            if let Some(debug) = debug_interface.as_mut() {
                for x in enemy_group {
                    let color = match sim_res {
                        WON(_) => TRANSPARENT_GREEN,
                        DRAW => TRANSPARENT_ORANGE,
                        LOST => TRANSPARENT_BLACK,
                    };
                    debug.add_circle(x.position, 1.5, color);
                }
            }
            sims.push(FightSim {
                result: sim_res,
                allies: my_group.iter().map(|e| e.id).collect(),
                enemies: enemy_group.iter().map(|e| e.id).collect(),
            });
        }
    }
    return sims;
}

fn find_enemy_groups() -> Vec<Vec<&'static Unit>> {
    let mut groups = Vec::new();
    for (_, units) in &get_all_enemy_units()
        .iter()
        .filter(|e| e.remaining_spawn_time.is_none())
        .group_by(|e| e.player_id)
    {
        groups.push(units.collect_vec());
    }
    groups
}

fn get_my_other_units_nearby(unit: &Unit) -> Vec<&Unit> {
    unit.my_other_units()
        .into_iter()
        .filter(|e| e.remaining_spawn_time.is_none())
        .filter(|e| e.position.distance(&unit.position) < GROUP_RADIUS)
        .collect_vec()
}

struct FightProps {
    position: Vec2,
    weapon: WeaponProperties,
    ammo: i32,
    aim: f64,
    health: f64,
    fire_tick: i32,
}

impl fmt::Display for FightProps {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "tick {}, rate {}, ammo {}, health {}",
            self.fire_tick,
            self.weapon.get_fire_rate_in_ticks(),
            self.ammo,
            self.health
        )
    }
}

pub fn simulation(u1: &Vec<&Unit>, u2: &Vec<&Unit>) -> FightSimResult {
    fn fighter_props(team: &Vec<&Unit>) -> Vec<FightProps> {
        team.iter()
            .filter(|e| e.weapon.is_some())
            .filter(|e| e.ammo[e.weapon.unwrap() as usize] != 0)
            .map(|e| FightProps {
                position: e.position,
                weapon: get_constants().weapons[e.weapon.unwrap() as usize].clone(),
                aim: e.aim,
                ammo: e.ammo[e.weapon.unwrap() as usize],
                health: e.health + e.shield,
                fire_tick: -(max(get_game().current_tick, e.next_shot_tick)
                    - get_game().current_tick),
                // fire_tick: 0,
            })
            .collect_vec()
    }
    let mut allies = fighter_props(u1);
    if allies.is_empty() {
        return LOST;
    }
    let mut enemies = fighter_props(u2);
    if enemies.is_empty() {
        return WON(0);
    }

    // println!("START!");
    while !allies.is_empty() && !enemies.is_empty() {
        // println!("{}, enemies {}",
        //          allies.iter().map(|e| e.to_string()).join(","),
        //          enemies.iter().map(|e| e.to_string()).join(","));
        let min_fire_rate = allies
            .iter()
            .map(|e| e.weapon.get_fire_rate_in_ticks())
            .chain(enemies.iter().map(|e| e.weapon.get_fire_rate_in_ticks()))
            .min()
            .unwrap();
        fn process_tick(
            fire_rate: i32,
            cur_team: &mut Vec<FightProps>,
            other_team: &mut Vec<FightProps>,
        ) {
            for mut ally in cur_team {
                if ally.ammo == 0 {
                    continue;
                }
                ally.fire_tick += fire_rate;
                if ally.fire_tick >= ally.weapon.get_fire_rate_in_ticks() {
                    ally.fire_tick -= ally.weapon.get_fire_rate_in_ticks();
                    for mut enemy in &mut *other_team {
                        if enemy.health > 0.0 {
                            enemy.health -= ally.weapon.projectile_damage
                                * chance_to_hit(&enemy.position, &ally.position, &enemy.weapon);
                            break;
                        }
                    }
                    ally.ammo -= 1
                }
            }
        }
        process_tick(min_fire_rate, &mut allies, &mut enemies);
        process_tick(min_fire_rate, &mut enemies, &mut allies);

        let mut any_with_ammo = false;
        for i in (0..allies.len()).rev() {
            any_with_ammo = any_with_ammo || allies[i].ammo != 0;
            if allies[i].health <= 0.0 {
                allies.remove(i);
            }
        }
        for i in (0..enemies.len()).rev() {
            any_with_ammo = any_with_ammo || enemies[i].ammo != 0;
            if enemies[i].health <= 0.0 {
                enemies.remove(i);
            }
        }
        if !any_with_ammo {
            return DRAW;
        };
    }
    // println!("END!");

    if enemies.is_empty() {
        if allies.is_empty() {
            DRAW
        } else {
            WON((u1.len() - allies.len()) as i32)
        }
    } else {
        LOST
    }
}

fn chance_to_hit(shooter_pos: &Vec2, target_pos: &Vec2, weapon: &WeaponProperties) -> f64 {
    let distance = shooter_pos.distance(target_pos);
    let angle = (target_pos.clone() - shooter_pos.clone()).angle();
    let left = rotate(shooter_pos.clone(), angle - weapon.spread / 2.0, distance);
    let right = rotate(shooter_pos.clone(), angle + weapon.spread / 2.0, distance);
    2.0 * get_constants().unit_radius / (left.distance(target_pos) + right.distance(target_pos))
}
