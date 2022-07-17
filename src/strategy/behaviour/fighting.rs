use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, RED, TEAL, TRANSPARENT_BLUE, TRANSPARENT_TEAL};
use crate::model::ActionOrder::Aim;
use crate::model::{Obstacle, Unit, UnitOrder, Vec2, WeaponProperties};
use crate::strategy::behaviour::behaviour::{write_behaviour, Behaviour, zone_penalty, my_units_magnet_score, my_units_collision_score};
use crate::strategy::holder::{get_constants, get_game, get_obstacles, get_all_enemy_units};
use crate::strategy::util::{bullet_trace_score, intersects_with_obstacles, intersects_with_obstacles_vec, get_projectile_traces, intersects_with_units_vec};
use itertools::{all, Itertools};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, Pointer};
use std::path::Display;

pub struct Fighting {}

impl Behaviour for Fighting {
    fn should_use(&self, unit: &Unit) -> bool {
        if unit.action.is_some() {
            return false;
        };
        let have_weapon_and_ammo = match unit.weapon {
            None => false,
            Some(weapon) => unit.ammo[weapon as usize] != 0,
        };
        if !have_weapon_and_ammo {
            return false;
        };

        let my_team = find_my_team(&unit);
        let enemy_groups = find_enemy_groups(&unit);

        enemy_groups
            .iter()
            .find(|e| can_win(&my_team, e, get_all_enemy_units().len() <= my_team.len()))
            // .find(|e| {
            //     e.position.distance(&unit.position) - get_constants().unit_radius
            //         < get_constants().weapons[unit.weapon.unwrap() as usize].firing_distance()
            // })
            .is_some()
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        write_behaviour(unit, "Fighting".to_owned(), debug_interface);

        let game = get_game();
        let constants = get_constants();
        let weapon = &constants.weapons[unit.weapon.unwrap_or(0) as usize];
        let traces = get_projectile_traces();
        let my_team = find_my_team(&unit);
        let enemy_groups = find_enemy_groups(&unit);

        if let Some(debug) = debug_interface.as_mut() {
            for t in &my_team {
                debug.add_circle(t.position.clone(), 0.5, TRANSPARENT_TEAL.clone());
            }
        }

        let target = enemy_groups
            .iter()
            .filter(|e| can_win(&my_team, e, e.len() <= my_team.len()))
            .flatten()
            .map(|e| (e, e.position.distance(&unit.position)))
            .min_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap())
            // .find(|e| {
            //     e.position.distance(&unit.position) - get_constants().unit_radius
            //         < get_constants().weapons[unit.weapon.unwrap() as usize].firing_distance()
            // })
            .unwrap().0;
        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(target.position.clone(), 0.5, RED.clone());
        }

        let obstacles = &get_obstacles(unit.id);
        let fire_target = target.position.clone()
            + (target.velocity.clone() * unit.position.distance(&target.position)
            / weapon.projectile_speed);

        let intersects_with_obstacles = intersects_with_obstacles_vec(
            &unit.position,
            &fire_target,
            obstacles,
        );
        let intersects_with_friends = intersects_with_units_vec(
            &unit.position,
            &fire_target,
            &unit.my_other_units(),
        );
        let goal = get_best_firing_spot(unit, &target, obstacles);

        let result_move = unit
            .points_around_unit(true)
            .iter()
            .map(|p| (p, bullet_trace_score(&traces, &p) + my_units_collision_score(&p, unit) + p.distance(&goal)))
            .min_by(|e1, e2| f64::partial_cmp(&e1.1, &e2.1).unwrap())
            .unwrap()
            .0
            .clone();

        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(result_move.clone(), 0.1, BLUE.clone());
            debug.add_circle(goal.clone(), 1.0, TRANSPARENT_BLUE.clone());
        }

        let ticks_until_next_shot =
            max(get_game().current_tick, unit.next_shot_tick) - get_game().current_tick;
        let action =
            if ticks_until_next_shot as f64 <= weapon.ticks_to_aim() as f64 * (1.0 - unit.aim) {
                Some(Aim {
                    shoot: !intersects_with_friends && !intersects_with_obstacles && unit.is_inside_vision(&target.position),
                })
            } else {
                None
            };

        UnitOrder {
            target_velocity: (result_move - unit.position.clone()) * 1000.0,
            target_direction: fire_target - unit.position.clone(),
            action: action,
        }
    }
}

fn find_enemy_groups(unit: &Unit) -> Vec<Vec<&Unit>> {
    let mut groups = Vec::new();
    for (_, units) in &get_all_enemy_units()
        .iter()
        .filter(|e| e.remaining_spawn_time.is_none())
        .group_by(|e| e.player_id) {
        groups.push(units.collect_vec());
    }
    groups
}

fn find_my_team(unit: &Unit) -> Vec<&Unit> {
    unit.my_other_units().into_iter()
        .filter(|e| e.position.distance(&unit.position) < 10.0).collect_vec()
}

fn get_best_firing_spot(unit: &Unit, target: &&Unit, obstacles: &Vec<Obstacle>) -> Vec2 {
    let mut best_point = Vec2::default();
    let mut best_score = f64::MIN;
    let constants = get_constants();
    for p in unit.points_in_radius(10) {
        if obstacles.iter().find(|o| o.position.distance(&p) < o.radius + constants.unit_radius).is_some() {
            continue;
        }
        if unit.my_other_units().iter().find(|o| o.position.distance(&p) < constants.unit_radius * 4.0).is_some() {
            continue;
        }
        let (units_in_firing_distance, units_not_in_firing_distance): (Vec<_>, Vec<_>) =
            get_all_enemy_units()
                .iter()
                .filter(|e| e.id != target.id)
                .partition(|e| {
                    e.position.distance(&p) - get_constants().unit_radius < e.firing_distance()
                        || intersects_with_obstacles_vec(&e.position, &p, obstacles)
                });
        let has_obstacles = intersects_with_obstacles_vec(&unit.position, &p, obstacles) ||
            intersects_with_units_vec(&unit.position, &p, &unit.my_other_units());

        let distance_to_target = p.distance(&target.position);
        let distance_score = (distance_to_target - unit.firing_distance() * 0.5).abs();

        // more is better
        let score = units_not_in_firing_distance.len() as f64 * 2.0
            - units_in_firing_distance.len() as f64 * 2.0
            - my_units_magnet_score(&p, unit)
            + if has_obstacles { -5.0 } else { 5.0 }
            - zone_penalty(&p)
            - distance_score;
        if best_score < score {
            best_score = score;
            best_point = p;
        }
    }
    best_point
}

struct FightProps {
    weapon: WeaponProperties,
    ammo: i32,
    aim: f64,
    health: f64,
    fire_tick: i32,
}

impl fmt::Display for FightProps {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tick {}, rate {}, ammo {}, health {}", self.fire_tick, self.weapon.get_fire_rate_in_ticks(), self.ammo, self.health)
    }
}

pub fn can_win(u1: &Vec<&Unit>, u2: &Vec<&Unit>, allow_draw: bool) -> bool {
    fn fighter_props(team: &Vec<&Unit>) -> Vec<FightProps> {
        team.iter()
            .filter(|e| e.weapon.is_some())
            .filter(|e| e.ammo[e.weapon.unwrap() as usize] != 0)
            .map(|e| FightProps {
                weapon: get_constants().weapons[e.weapon.unwrap() as usize].clone(),
                aim: e.aim,
                ammo: e.ammo[e.weapon.unwrap() as usize],
                health: e.health + e.shield,
                fire_tick: -(max(get_game().current_tick, e.next_shot_tick) - get_game().current_tick),
            })
            .collect_vec()
    }
    let mut allies = fighter_props(u1);
    if allies.is_empty() {
        return false;
    }
    let mut enemies = fighter_props(u2);
    if enemies.is_empty() {
        return true;
    }

    // println!("START!");
    while !allies.is_empty() && !enemies.is_empty() {
        // println!("{}, enemies {}",
        //          allies.iter().map(|e| e.to_string()).join(","),
        //          enemies.iter().map(|e| e.to_string()).join(","));
        let min_fire_rate =
            allies.iter().map(|e| e.weapon.get_fire_rate_in_ticks()).chain(enemies.iter().map(|e| e.weapon.get_fire_rate_in_ticks()))
                .min().unwrap();
        fn process_tick(fire_rate: i32, cur_team: &mut Vec<FightProps>, other_team: &mut Vec<FightProps>) {
            for mut ally in cur_team {
                ally.fire_tick += fire_rate;
                if ally.fire_tick >= ally.weapon.get_fire_rate_in_ticks() {
                    ally.fire_tick -= ally.weapon.get_fire_rate_in_ticks();
                    for mut enemy in &mut *other_team {
                        if enemy.health > 0.0 {
                            enemy.health -= ally.weapon.projectile_damage;
                            break;
                        }
                    }
                    ally.ammo -= 1
                }
            }
        }
        process_tick(min_fire_rate, &mut allies, &mut enemies);
        process_tick(min_fire_rate, &mut enemies, &mut allies);

        for i in (0..allies.len()).rev() {
            if allies[i].ammo == 0 || allies[i].health <= 0.0 {
                allies.remove(i);
            }
        }
        for i in (0..enemies.len()).rev() {
            if enemies[i].ammo == 0 || enemies[i].health <= 0.0 {
                enemies.remove(i);
            }
        }
    }
    // println!("END!");
    return (!allies.is_empty() || allow_draw) && enemies.is_empty();
}
