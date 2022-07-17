use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, RED, TEAL, TRANSPARENT_BLUE, TRANSPARENT_TEAL};
use crate::model::ActionOrder::Aim;
use crate::model::{Obstacle, Unit, UnitOrder, Vec2, WeaponProperties};
use crate::strategy::behaviour::behaviour::{
    my_units_collision_score, my_units_magnet_score, write_behaviour, zone_penalty, Behaviour,
};
use crate::strategy::holder::fight_sim::FightSimResult;
use crate::strategy::holder::{
    get_all_enemy_units, get_constants, get_fight_simulations, get_game, get_obstacles,
};
use crate::strategy::util::{
    bullet_trace_score, get_projectile_traces, intersects_with_obstacles,
    intersects_with_obstacles_vec, intersects_with_units_vec,
};
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

        let fight_sims = get_fight_simulations();
        fight_sims
            .into_iter()
            .filter(|s| {
                s.allies.contains(&unit.id)
                    && match s.result {
                        FightSimResult::WON(_) => true,
                        FightSimResult::DRAW => true,
                        FightSimResult::LOST => false,
                    }
            })
            .flat_map(|e| e.enemy_units())
            .find(|e| e.position.distance(&unit.position) < unit.firing_distance())
            .is_some()
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        write_behaviour(unit, "Fighting".to_owned(), debug_interface);

        let game = get_game();
        let constants = get_constants();
        let weapon = &constants.weapons[unit.weapon.unwrap_or(0) as usize];
        let traces = get_projectile_traces();
        let fight_sims = get_fight_simulations();

        let targets = fight_sims
            .into_iter()
            .filter(|s| {
                s.allies.contains(&unit.id)
                    && match s.result {
                        FightSimResult::WON(_) => true,
                        FightSimResult::DRAW => true,
                        FightSimResult::LOST => false,
                    }
            })
            .flat_map(|s| s.enemy_units())
            .map(|e| (e, e.position.distance(&unit.position)))
            .collect_vec();

        let target = targets
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(target.position.clone(), 0.5, RED.clone());
        }

        let obstacles = &get_obstacles(unit.id);
        let fire_target = target.position.clone()
            + (target.velocity.clone() * unit.position.distance(&target.position)
                / weapon.projectile_speed);

        let intersects_with_obstacles =
            intersects_with_obstacles_vec(&unit.position, &fire_target, obstacles);
        let intersects_with_friends =
            intersects_with_units_vec(&unit.position, &fire_target, &unit.my_other_units());
        let goal = get_best_firing_spot(unit, &target, obstacles);

        let result_move = unit
            .points_around_unit(true)
            .iter()
            .map(|p| {
                (
                    p,
                    bullet_trace_score(&traces, &p)
                        + my_units_collision_score(&p, unit)
                        + p.distance(&goal),
                )
            })
            .min_by(|e1, e2| f64::partial_cmp(&e1.1, &e2.1).unwrap())
            .map(|e| e.0)
            //TODO
            .unwrap_or(&game.zone.current_center)
            .clone();

        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(result_move.clone(), 0.1, BLUE.clone());
            debug.add_circle(goal.clone(), 1.0, TRANSPARENT_BLUE.clone());
        }

        let ticks_until_next_shot =
            max(get_game().current_tick, unit.next_shot_tick) - get_game().current_tick;
        let action = if fire_target.distance(&unit.position) < weapon.firing_distance()
            && ticks_until_next_shot as f64 <= weapon.ticks_to_aim() as f64 * (1.0 - unit.aim)
        {
            Some(Aim {
                shoot: !intersects_with_friends
                    && !intersects_with_obstacles
                    && unit.is_inside_vision(&target.position),
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
        if obstacles
            .iter()
            .find(|o| o.position.distance(&p) < o.radius + constants.unit_radius)
            .is_some()
        {
            continue;
        }
        if unit
            .my_other_units()
            .iter()
            .find(|o| o.position.distance(&p) < constants.unit_radius * 4.0)
            .is_some()
        {
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
        let has_obstacles = intersects_with_obstacles_vec(&unit.position, &p, obstacles)
            || intersects_with_units_vec(&unit.position, &p, &unit.my_other_units());

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
