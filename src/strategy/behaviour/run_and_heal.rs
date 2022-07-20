use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, GREEN, RED, TRANSPARENT_BLUE};
use crate::model::ActionOrder::UseShieldPotion;
use crate::model::{Unit, UnitOrder, Vec2};
use crate::strategy::behaviour::behaviour::{my_units_magnet_score, write_behaviour, Behaviour};
use crate::strategy::holder::fight_sim::FightSimResult;
use crate::strategy::holder::{
    get_all_enemy_units, get_constants, get_fight_simulations, get_game, get_obstacles,
};
use crate::strategy::util::{
    bullet_trace_score, get_projectile_traces, intersects_with_obstacles, rotate,
};
use std::env::set_current_dir;

pub struct RunAndHeal {}

impl Behaviour for RunAndHeal {
    fn should_use(&self, unit: &Unit) -> bool {
        let any_sim_lost = get_fight_simulations().into_iter().any(|s| {
            s.allies.contains(&unit.id)
                && s.enemy_units()
                    .iter()
                    .any(|e| e.position.distance(&unit.position) <= e.firing_distance())
                && match s.result {
                    FightSimResult::WON(_) => false,
                    FightSimResult::DRAW => false,
                    FightSimResult::LOST => true,
                }
        });

        if any_sim_lost && get_game().current_tick < 5000 {
            return true;
        }

        unit.health < get_constants().unit_health * 0.5
            || unit.shield < get_constants().max_shield && unit.shield_potions > 0
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        write_behaviour(unit, "Run".to_owned(), debug_interface);

        let mut top_score: f64 = f64::MAX;
        let mut goal: Vec2 = get_game().zone.current_center.clone();
        let obstacles = get_obstacles(unit.id);
        let traces = get_projectile_traces();
        for p in unit.points_in_radius(10) {
            if obstacles
                .iter()
                .find(|o| o.position.distance(&p) < o.radius + get_constants().unit_radius)
                .is_some()
            {
                continue;
            }
            if get_game().zone.current_center.distance(&p) + 3.0 >= get_game().zone.current_radius {
                continue;
            }
            if let Some(debug) = debug_interface.as_mut() {
                debug.add_circle(p.clone(), 0.1, RED.clone());
            }
            let enemy_score = get_all_enemy_units()
                .iter()
                .map(|e| e.position.distance(&p))
                .min_by_key(|s| s.ceil() as i64)
                .unwrap_or(0.0);
            let distance_from_previous_score = if p.distance(&unit.position) > 3.0 {
                3.0
            } else {
                p.distance(&unit.position)
            };
            let res = -enemy_score - distance_from_previous_score
                + (my_units_magnet_score(&p, unit) / 2.0);
            if res < top_score {
                goal = p;
                top_score = res;
            }
        }

        let result_move = unit
            .points_around_unit(true)
            .iter()
            .map(|e| (e, bullet_trace_score(&traces, &e) + e.distance(&goal)))
            .min_by(|e1, e2| f64::partial_cmp(&e1.1, &e2.1).unwrap())
            .unwrap()
            .0
            .clone();

        let rotation = if get_game().current_tick % 100 >= 85 {
            Vec2 {
                x: -unit.direction.y,
                y: unit.direction.x,
            }
        } else {
            goal.clone() - unit.position.clone()
        };

        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(result_move.clone(), 0.1, BLUE.clone());
            debug.add_circle(goal.clone(), 1.0, RED.clone());
            for x in get_projectile_traces() {
                debug.add_circle(x.position, 0.1, BLUE.clone());
            }
        }
        UnitOrder {
            target_velocity: (result_move - unit.position.clone()) * 1000.0,
            target_direction: rotation,
            action: Some(UseShieldPotion {}),
        }
    }
}
