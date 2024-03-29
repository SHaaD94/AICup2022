pub mod behaviour;
pub mod holder;
pub mod loot;
pub mod potential_field;
pub mod util;

use crate::debug_interface::DebugInterface;
use crate::debugging::GREEN;
use crate::model;
use crate::model::ActionOrder::Aim;
use crate::model::{ActionOrder, Constants, Game, Item, Loot, Unit, UnitOrder, Vec2};
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::behaviour::fighting::Fighting;
use crate::strategy::behaviour::ghosting::Ghosting;
use crate::strategy::behaviour::move_or_loot::MoveOrLoot;
use crate::strategy::behaviour::run_and_heal::RunAndHeal;
use crate::strategy::holder::{get_constants, get_game};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Index;

pub fn get_order(debug_interface: &mut Option<&mut DebugInterface>) -> model::Order {
    let game = get_game();
    let constants = get_constants();

    let behaviours: Vec<Box<dyn Behaviour>> = vec![
        Box::new(Ghosting {}),
        Box::new(Fighting {}),
        Box::new(RunAndHeal {}),
        Box::new(MoveOrLoot {}),
    ];

    let orders: HashMap<i32, UnitOrder> = game
        .my_units()
        .into_iter()
        .sorted_by_key(|e| e.id)
        .map(|u| {
            let mut order: UnitOrder = UnitOrder {
                target_velocity: Default::default(),
                target_direction: Default::default(),
                action: None,
            };
            if let Some(debug) = debug_interface.as_mut() {
                // debug.add_circle(u.position.clone(), 23.0, GREEN.clone())
            }
            for behaviour in &behaviours {
                if behaviour.should_use(u) {
                    order = behaviour.order(u, debug_interface);
                    break;
                }
            }
            (u.id, order)
        })
        .collect();

    model::Order {
        unit_orders: orders,
    }
}
