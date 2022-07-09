pub mod util;

use std::collections::HashMap;
use itertools::Itertools;
use crate::model;
use crate::model::{Constants, Game, Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Aim;

pub fn get_order(game: &Game, constants: &Constants) -> model::Order {
    let orders: HashMap<i32, UnitOrder> = game.my_units().into_iter().map(|u| {
        let id = u.id;
        // println!("{} {}", game.zone.current_center.x, game.zone.current_center.y);
        // println!("{} {}", u.position.x, u.position.y);
        let move_target = Vec2 {
            x: game.zone.next_center.x,
            y: game.zone.next_center.y,
        };
        let target_direction = game.enemy_units().iter().min_by_key(|e| e.position.distance(&u.position).ceil() as i64).map(|it| it.position.clone());
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