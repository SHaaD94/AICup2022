pub mod util;
pub mod holder;
mod behaviour;
pub mod loot;
pub mod potential_field;

use std::collections::HashMap;
use std::ops::Index;
use itertools::Itertools;
use crate::model;
use crate::model::{ActionOrder, Constants, Game, Item, Loot, Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Aim;
use crate::strategy::holder::{get_constants, get_game};
use crate::strategy::behaviour::{Behaviour, Fighting, MoveToCenterOrLoot, UseHeal};

pub fn get_order() -> model::Order {
    let game = get_game();
    let constants = get_constants();

    let behaviours: Vec<Box<dyn Behaviour>> = vec![
        Box::new(UseHeal {}),
        Box::new(Fighting {}),
        Box::new(MoveToCenterOrLoot {})];

    let orders: HashMap<i32, UnitOrder> = game.my_units().into_iter().map(|u| {
        let mut order: UnitOrder = UnitOrder {
            target_velocity: Default::default(),
            target_direction: Default::default(),
            action: None,
        };
        for behaviour in &behaviours {
            if behaviour.should_use(u) {
                order = behaviour.order(u);
                break;
            }
        }
        (u.id, order)
    }).collect();

    model::Order {
        unit_orders: orders,
    }
}
