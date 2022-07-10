use itertools::Itertools;
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Pickup;
use crate::strategy::holder::{get_constants, get_game};
use crate::strategy::loot::best_loot;

pub trait Behaviour: Sync {
    fn should_use(&self, unit: &Unit) -> bool;
    fn order(&self, unit: &Unit) -> UnitOrder;
}

pub struct MoveToCenterOrLoot {}

impl Behaviour for MoveToCenterOrLoot {
    fn should_use(&self, unit: &Unit) -> bool { true }

    fn order(&self, unit: &Unit) -> UnitOrder {
        let game = get_game();
        let constants = get_constants();
        let best_not_intersecting_loot = best_loot(unit, game.loot.iter().collect_vec(), false);
        let next_zone_center = Vec2 {
            x: game.zone.next_center.x,
            y: game.zone.next_center.y,
        };
        let best_intersecting_loot = best_loot(unit, game.loot.iter().collect_vec(), true);
        UnitOrder {
            target_velocity: (unit.position.clone() - best_not_intersecting_loot.as_ref().map(|l| l.position.clone()).unwrap_or(next_zone_center.clone())) * 1000.0,
            target_direction: unit.position.clone() - best_not_intersecting_loot.map(|l| l.position).unwrap_or(next_zone_center),
            action: best_intersecting_loot.map(|l| Pickup { loot: l.id }),
        }
    }
}

pub struct Fighting {}

impl Behaviour for Fighting {
    fn should_use(&self, unit: &Unit) -> bool { true }

    fn order(&self, unit: &Unit) -> UnitOrder {
        let game = get_game();
        let constants = get_constants();
        UnitOrder {
            target_velocity: Default::default(),
            target_direction: Default::default(),
            action: None,
        }
    }
}

// pub struct LookAround {}
//
// impl Behaviour for LookAround {
//     fn should_use(&self, unit: &Unit) -> bool { true }
//
//     fn order(&self, unit: &Unit) -> UnitOrder {
//         let game = get_game();
//         let constants = get_constants();
//         UnitOrder {
//             target_velocity: Default::default(),
//             target_direction: Default::default(),
//             action: None,
//         }
//     }
// }

