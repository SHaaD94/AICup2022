pub mod util;
pub mod holder;

use std::collections::HashMap;
use itertools::Itertools;
use crate::model;
use crate::model::{Constants, Game, Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Aim;
use crate::strategy::holder::get_game;

pub fn get_order() -> model::Order {
    println!("{}",get_game().current_tick);

    let orders: HashMap<i32, UnitOrder> = get_game().my_units().into_iter().map(|u| {
        let id = u.id;
        let move_target = Vec2 {
            x: get_game().zone.next_center.x,
            y: get_game().zone.next_center.y,
        };
        let target_direction = get_game().enemy_units().iter()
            .min_by_key(|e| (e.position.distance(&u.position) * 1000.0).ceil() as i64)
            .map(|it| it.position.clone());
        let order = UnitOrder {
            target_velocity: u.position.clone() - move_target.clone(),
            action: Some(Aim {
                shoot: target_direction.is_some()
            }),
            target_direction: u.position.clone() - target_direction.unwrap_or(move_target),
        };
        (id, order)
    }).collect();

    model::Order {
        unit_orders: orders,
    }
}



