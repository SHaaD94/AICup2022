use crate::debug_interface::DebugInterface;
use crate::debugging::RED;
use crate::model::ActionOrder::{Aim, Pickup, UseShieldPotion};
use crate::model::Item::ShieldPotions;
use crate::model::{ActionOrder, Loot, Unit, UnitOrder, Vec2};
use crate::strategy::holder::{
    get_constants, get_game, get_loot, get_obstacles, get_all_enemy_units, remove_loot,
};
use crate::strategy::loot::best_loot;
use crate::strategy::util::intersects_with_obstacles;
use itertools::Itertools;
use std::cmp::min;

pub trait Behaviour: Sync {
    fn should_use(&self, unit: &Unit) -> bool;
    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder;
}

pub fn write_behaviour(
    unit: &Unit,
    text: String,
    debug_interface: &mut Option<&mut DebugInterface>,
) {
    if let Some(debug) = debug_interface.as_mut() {
        let result_text = format!("{}, {}, {}, {}", text, unit.extra_lives, unit.weapon.unwrap_or(0), unit.ammo_for_current_weapon());
        debug.add_placed_text(
            unit.position.clone() - Vec2 { x: 0.0, y: -5.0 },
            result_text,
            Vec2 { x: 1.0, y: 1.0 },
            1.0,
            RED.clone(),
        )
    }
}

pub fn zone_penalty(p: &Vec2) -> f64 {
    let distance_to_zone_center = p.distance(&get_game().zone.current_center);
    let zone_penalty_score = if distance_to_zone_center / &get_game().zone.current_radius > 0.9
    {
        distance_to_zone_center * 50.0
    } else {
        0.0
    };
    zone_penalty_score
}

// more is worse
pub fn my_units_collision_score(p: &Vec2, unit: &Unit) -> f64 {
    match unit.my_closest_other_unit() {
        None => 0.0,
        Some(other) => {
            let distance = other.1.position.distance(p);
            if distance <= 5.0 {
                5.0 - distance
            } else {
                0.0
            }
        }
    }
}

pub fn my_units_magnet_score(p: &Vec2, unit: &Unit) -> f64 {
    match unit.my_closest_other_unit() {
        None => 0.0,
        Some(other) => {
            let distance = other.1.position.distance(p);
            distance * 0.5
        }
    }
}
