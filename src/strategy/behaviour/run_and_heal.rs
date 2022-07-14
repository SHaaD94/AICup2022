use std::env::set_current_dir;
use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, GREEN, RED, TRANSPARENT_BLUE};
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::UseShieldPotion;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::behaviour::fighting::simulation;
use crate::strategy::holder::{get_constants, get_game, get_obstacles, get_units};
use crate::strategy::util::{bullet_trace_score, does_intersect, get_projectile_traces, rotate};

pub struct RunAndHeal {}

impl Behaviour for RunAndHeal {
    fn should_use(&self, unit: &Unit) -> bool {
        // if get_units().iter()
        //     .filter(|e| {
        //         let distance = get_constants().weapons[e.weapon.unwrap_or(0) as usize].firing_distance();
        //         e.position.distance(&unit.position) < distance
        //     })
        //     .find(|e| !simulation(unit, e)).is_some() { return true; };

        unit.health < get_constants().unit_health * 0.5 || unit.shield < get_constants().max_shield && unit.shield_potions > 0
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        if let Some(debug) = debug_interface.as_mut() {
            debug.add_placed_text(
                unit.position.clone() - Vec2 { x: 0.0, y: -5.0 },
                "Running".to_owned(),
                Vec2 { x: 1.0, y: 1.0 },
                1.0,
                RED.clone(),
            )
        }

        let mut top_score: f64 = f64::MAX;
        let mut goal: Vec2 = Vec2::default();
        let obstacles = get_obstacles(unit.id);
        let traces = get_projectile_traces();
        for x in 0..21 {
            for y in 0..21 {
                let p = Vec2 { x: unit.position.x + x as f64 - 10.0, y: unit.position.y + y as f64 - 10.0 };
                if unit.position.distance(&p) > 10.0 { continue; }
                if obstacles.iter().find(|o| o.position.distance(&p) < o.radius + get_constants().unit_radius).is_some() {
                    continue;
                }
                if get_game().zone.current_center.distance(&p) + 5.0 >= get_game().zone.current_radius {
                    continue;
                }
                if let Some(debug) = debug_interface.as_mut() {
                    debug.add_circle(p.clone(), 0.1, RED.clone());
                }
                let enemy_score = get_units().iter().map(|e| {
                    e.position.distance(&p)
                }).min_by_key(|s| s.ceil() as i64).unwrap_or(0.0);
                let distance_from_previous_score =
                    if p.distance(&unit.position) > 3.0 {
                        3.0
                    } else {
                        p.distance(&unit.position)
                    };
                let res = -enemy_score
                    - distance_from_previous_score;
                if res < top_score {
                    goal = p;
                    top_score = res;
                }
            }
        }

        let result_move = unit.points_around_unit().iter()
            .map(|e| (e, bullet_trace_score(&traces, &e) + e.distance(&goal)))
            .min_by(|e1, e2| {
                f64::partial_cmp(&e1.1, &e2.1).unwrap()
            }).unwrap().0.clone();

        let rotation = if get_game().current_tick % 100 >= 85 {
            Vec2 { x: -unit.direction.y, y: unit.direction.x }
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