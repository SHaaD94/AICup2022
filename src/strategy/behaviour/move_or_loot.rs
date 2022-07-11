use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, GREEN, TRANSPARENT_BLUE, TRANSPARENT_GREEN};
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Pickup;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::holder::{get_constants, get_game, get_loot, remove_loot};
use crate::strategy::loot::best_loot;

pub struct MoveToCenterOrLoot {}

impl Behaviour for MoveToCenterOrLoot {
    fn should_use(&self, unit: &Unit) -> bool { true }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        let game = get_game();
        let constants = get_constants();
        let loot = get_loot();
        let best_not_intersecting_loot = best_loot(unit, loot, false);
        let next_zone_center = Vec2 {
            x: game.zone.next_center.x,
            y: game.zone.next_center.y,
        };
        let best_intersecting_loot = best_loot(unit, loot, true);
        if let Some(loot) = &best_intersecting_loot {
            unsafe { remove_loot(loot.id.clone()); }
        }
        let move_target = best_not_intersecting_loot.map(|l| l.position).unwrap_or(next_zone_center);
        let result_move = unit.points_around_unit().iter()
            .min_by_key(|e| (-e.distance(&unit.position) + e.distance(&move_target).ceil() * 1000.0) as i32).unwrap().clone();
        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(result_move.clone(), 1.0, TRANSPARENT_BLUE.clone());
            debug.add_circle(move_target.clone(), -0.5, TRANSPARENT_BLUE.clone());
        }
        UnitOrder {
            target_velocity: (result_move.clone() - unit.position.clone()) * 1000.0,
            target_direction: result_move.clone() - unit.position.clone(),
            // target_direction: move_target.clone() - unit.position.clone(),
            action: best_intersecting_loot.map(|l| Pickup { loot: l.id }),
        }
    }
}
