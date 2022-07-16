use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, RED, TRANSPARENT_BLUE};
use crate::model::ActionOrder::Aim;
use crate::model::{Obstacle, Unit, UnitOrder, Vec2};
use crate::strategy::behaviour::behaviour::{write_behaviour, Behaviour, zone_penalty, my_units_magnet_score, my_units_collision_score};
use crate::strategy::holder::{get_constants, get_game, get_obstacles, get_all_enemy_units};
use crate::strategy::util::{bullet_trace_score, intersects_with_obstacles, intersects_with_obstacles_vec, get_projectile_traces, intersects_with_units_vec};
use itertools::{all, Itertools};
use std::cmp::{max, min};

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

        get_all_enemy_units()
            .iter()
            .filter(|e| simulation(unit, e, get_all_enemy_units().len() == 1))
            .find(|e| {
                e.position.distance(&unit.position) - get_constants().unit_radius
                    < get_constants().weapons[unit.weapon.unwrap() as usize].firing_distance()
            })
            .is_some()
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        write_behaviour(unit, "Fighting".to_owned(), debug_interface);

        let game = get_game();
        let constants = get_constants();
        let weapon = &constants.weapons[unit.weapon.unwrap_or(0) as usize];
        let traces = get_projectile_traces();
        let target = get_all_enemy_units()
            .iter()
            .filter(|e| simulation(unit, e, get_all_enemy_units().len() == 1))
            .min_by_key(|e| e.position.distance(&unit.position) as i64)
            .unwrap();

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
            .map(|p| (p, bullet_trace_score(&traces, &p) /*+ my_units_collision_score(&p, unit)*/ + p.distance(&goal)))
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

pub fn simulation(u1: &Unit, u2: &Unit, allow_draw: bool) -> bool {
    if u1.weapon.is_none() {
        return false;
    }
    if u2.weapon.is_none() {
        return true;
    }
    let constants = get_constants();
    let mut ammo1 = u1.ammo[u1.weapon.unwrap() as usize];
    let w1 = &constants.weapons[u1.weapon.unwrap() as usize];
    let mut h1 = u1.health + u1.shield;
    let mut ammo2 = u2.ammo[u2.weapon.unwrap() as usize];
    let w2 = &constants.weapons[u2.weapon.unwrap() as usize];
    let mut h2 = u2.health + u2.shield;

    let mut tick1 = -(max(get_game().current_tick, u1.next_shot_tick) - get_game().current_tick);
    let mut tick2 = -(max(get_game().current_tick, u2.next_shot_tick) - get_game().current_tick);
    tick1 += ((u1.aim - 1.0) * (w1.aim_time) * constants.ticks_per_second).ceil() as i32;
    tick2 += ((u2.aim - 1.0) * (w2.aim_time) * constants.ticks_per_second).ceil() as i32;

    // println!("START!");
    while h1 >= 0.0 && h2 >= 0.0 {
        // println!("h1 {}, h2 {}, tick1 {}, tick2 {}, rate1 {}, rate2 {}, ammo1 {}, ammo2 {}",
        //          h1, h2, tick1, tick2, w1.get_fire_rate_in_ticks(), w2.get_fire_rate_in_ticks(), ammo1, ammo2);
        if ammo1 == 0 {
            h1 = 0.0;
            break;
        }
        if ammo2 == 0 {
            h2 = 0.0;
            break;
        }
        let min = min(w1.get_fire_rate_in_ticks(), w2.get_fire_rate_in_ticks());
        tick1 += min;
        tick2 += min;
        if tick1 >= w1.get_fire_rate_in_ticks() {
            tick1 -= w1.get_fire_rate_in_ticks();
            h2 -= w1.projectile_damage;
            ammo1 -= 1;
        }
        if tick2 >= w2.get_fire_rate_in_ticks() {
            tick2 -= w2.get_fire_rate_in_ticks();
            h1 -= w2.projectile_damage;
            ammo2 -= 1;
        }
    }
    // println!("END!");
    return (h1 > 0.0 || allow_draw) && h2 <= 0.0;
}
