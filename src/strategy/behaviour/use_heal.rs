use std::env::set_current_dir;
use crate::debug_interface::DebugInterface;
use crate::debugging::RED;
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::UseShieldPotion;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::holder::{get_constants, get_game, get_obstacles, get_units};
use crate::strategy::util::{does_intersect, rotate};

pub struct UseHeal {}

impl Behaviour for UseHeal {
    fn should_use(&self, unit: &Unit) -> bool {
        unit.health < get_constants().unit_health * 0.5 || unit.shield < get_constants().max_shield && unit.shield_potions > 0
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        let mut top_score: f64 = f64::MIN;
        let mut top_point: Vec2 = Vec2::default();
        let obstacles = get_obstacles(unit.id);
        for x in 0..21 {
            for y in 0..21 {
                let p = Vec2 { x: unit.position.x + x as f64 - 10.0, y: unit.position.y + y as f64 - 10.0 };
                if obstacles.iter().find(|o| o.position.distance(&p) < o.radius + get_constants().unit_radius).is_some() {
                    continue;
                }
                let enemy_score = get_units().iter().map(|e| {
                    if does_intersect(
                        p.x,
                        p.y,
                        e.position.x,
                        e.position.y,
                        &obstacles) {
                        100.0
                    } else {
                        e.position.distance(&p) * 10.0
                    }
                }).sum::<f64>();
                let area_penalty = if get_game().zone.current_center.distance(&p) >= get_game().zone.current_radius {
                    5000.0
                } else { 0.0 };
                let res = enemy_score - p.distance(&unit.position) - area_penalty;
                if enemy_score + p.distance(&unit.position) - area_penalty > top_score {
                    top_point = p;
                    top_score = res;
                }
            }
        }
        if let Some(debug) = debug_interface.as_mut() {
            // for p in points_to_check {
            //     debug.add_circle(p, 0.1, RED.clone())
            // }
            debug.add_circle(top_point.clone(), 1.0, RED.clone())
        }
        let rotation = if get_game().current_tick % 100 >= 85 {
            Vec2 { x: -unit.direction.y, y: unit.direction.x }
        } else {
            top_point.clone() - unit.position.clone()
        };
        UnitOrder {
            target_velocity: top_point - unit.position.clone(),
            target_direction: rotation,
            action: Some(UseShieldPotion {}),
        }
    }
}