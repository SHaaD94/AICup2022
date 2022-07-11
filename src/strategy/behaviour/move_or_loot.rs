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

    fn order(&self, unit: &Unit, mut debug_interface: Option<&mut DebugInterface>) -> UnitOrder {
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
        if let Some(debug) = debug_interface.as_mut() {
            if let Some(ref loot) = best_not_intersecting_loot {
                debug.add_circle(loot.position.clone(), 0.3, BLUE.clone());
            }
            for p in unit.points_around_unit() {
                debug.add_circle(p, 0.5, TRANSPARENT_BLUE.clone());
            }
        }

        UnitOrder {
            target_velocity: (best_not_intersecting_loot.as_ref().map(|l| l.position.clone()).unwrap_or(next_zone_center.clone()) - unit.position.clone()) * 1000.0,
            target_direction: best_not_intersecting_loot.map(|l| l.position).unwrap_or(next_zone_center) - unit.position.clone(),
            action: best_intersecting_loot.map(|l| Pickup { loot: l.id }),
        }
    }
}
