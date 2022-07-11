use crate::debug_interface::DebugInterface;
use crate::model::{Unit, UnitOrder};
use crate::model::ActionOrder::UseShieldPotion;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::holder::get_constants;

pub struct UseHeal {}

impl Behaviour for UseHeal {
    fn should_use(&self, unit: &Unit) -> bool {
        unit.shield < get_constants().max_shield && unit.shield_potions > 0
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        UnitOrder {
            target_velocity: unit.velocity.clone(),
            target_direction: unit.direction.clone(),
            action: Some(UseShieldPotion {}),
        }
    }
}