use std::env::set_current_dir;
use crate::debug_interface::DebugInterface;
use crate::debugging::RED;
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::UseShieldPotion;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::behaviour::fighting::simulation;
use crate::strategy::holder::{get_constants, get_game, get_obstacles, get_units};
use crate::strategy::util::{does_intersect, rotate};

pub struct RunAndHeal {}

impl Behaviour for RunAndHeal {
    fn should_use(&self, unit: &Unit) -> bool {
        if get_units().iter()
            //TODO check firing distance
            .find(|e| !simulation(unit, e)).is_some() { return true; };

        unit.health < get_constants().unit_health * 0.5 || unit.shield < get_constants().max_shield && unit.shield_potions > 0
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        let mut top_score: f64 = f64::MIN;
        let mut top_point: Vec2 = Vec2::default();
        let obstacles = get_obstacles(unit.id);
        for x in 0..21 {
            for y in 0..21 {
                let p = Vec2 { x: unit.position.x + x as f64 - 10.0, y: unit.position.y + y as f64 - 10.0 };
                if unit.position.distance(&p) > 10.0 { continue; }
                if obstacles.iter().find(|o| o.position.distance(&p) < o.radius + get_constants().unit_radius).is_some() {
                    continue;
                }
                let enemy_score = get_units().iter().map(|e| {
                    let firing_distance = e.weapon.map(|w| get_constants().weapons[w as usize].firing_distance())
                        .unwrap_or(0.0);
                    let distance = e.position.distance(&p);
                    if does_intersect(
                        p.x,
                        p.y,
                        e.position.x,
                        e.position.y,
                        &obstacles) || distance > firing_distance {
                        firing_distance
                    } else {
                        distance
                    }
                }).sum::<f64>();
                let area_penalty = if get_game().zone.current_center.distance(&p) + 10.0 >= get_game().zone.current_radius {
                    5000.0
                } else { 0.0 };
                let distance_from_previous_score = if p.distance(&unit.position) > 3.0 { 3.0 } else { p.distance(&unit.position) };
                let res = enemy_score + distance_from_previous_score - area_penalty;
                if res > top_score {
                    top_point = p;
                    top_score = res;
                }
            }
        }
        let result_move = unit.points_around_unit().iter()
            .min_by_key(|e| (e.distance(&unit.position) + e.distance(&top_point).ceil() * 1000.0) as i32).unwrap().clone();

        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(top_point.clone(), 1.0, RED.clone())
        }
        let rotation = if get_game().current_tick % 100 >= 85 {
            Vec2 { x: -unit.direction.y, y: unit.direction.x }
        } else {
            result_move.clone() - unit.position.clone()
        };
        UnitOrder {
            target_velocity: (result_move - unit.position.clone()) * 1000.0,
            target_direction: rotation,
            action: Some(UseShieldPotion {}),
        }
    }
}