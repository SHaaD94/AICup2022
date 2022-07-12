use std::cmp::min;
use itertools::Itertools;
use crate::debug_interface::DebugInterface;
use crate::model::{ActionOrder, Loot, Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::{Aim, Pickup, UseShieldPotion};
use crate::model::Item::ShieldPotions;
use crate::strategy::holder::{get_constants, get_game, get_loot, get_obstacles, get_units, remove_loot};
use crate::strategy::loot::best_loot;
use crate::strategy::util::does_intersect;

pub trait Behaviour: Sync {
    fn should_use(&self, unit: &Unit) -> bool;
    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder;
}

