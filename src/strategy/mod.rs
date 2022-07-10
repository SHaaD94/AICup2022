pub mod util;
pub mod holder;

use std::collections::HashMap;
use std::ops::Index;
use itertools::Itertools;
use crate::model;
use crate::model::{ActionOrder, Constants, Game, Item, Loot, Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Aim;
use crate::strategy::holder::{get_constants, get_game};

pub fn get_order() -> model::Order {
    let game = get_game();
    let constants = get_constants();

    let orders: HashMap<i32, UnitOrder> = game.my_units().into_iter().map(|u| {
        let intersected_loot = game.intersecting_loot(u);
        let valuable_loot = game.loot.iter().find_or_first(|l| item_filter(constants, u, l)).map(|e| e.clone());
        let loot_to_pick = intersected_loot.iter().find_or_first(|l| item_filter(constants, u, l));

        let target_direction = game.enemy_units().iter()
            .min_by_key(|e| (e.position.distance(&u.position) * 1000.0).ceil() as i64)
            .map(|it| it.position.clone());

        let action = {
            if loot_to_pick.is_some() {
                Some(ActionOrder::Pickup { loot: loot_to_pick.unwrap().id })
            } else if u.ammo[u.weapon.unwrap_or(0) as usize] == 0 {
                None
            } else {
                target_direction.clone().map(|e| Aim {
                    shoot: true
                })
            }
        };

        let move_target = valuable_loot.map(|e| e.position).unwrap_or(Vec2 {
            x: game.zone.next_center.x,
            y: game.zone.next_center.y,
        });
        println!("{}", u.position.clone() - move_target.clone());
        let order = UnitOrder {
            target_velocity: (u.position.clone() - move_target.clone()) * 1000.00,
            action,
            target_direction: u.position.clone() - target_direction.unwrap_or(move_target),
        };
        (u.id, order)
    }).collect();

    model::Order {
        unit_orders: orders,
    }
}

fn item_filter(constants: &Constants, u: &Unit, l: &Loot) -> bool {
    match l.item {
        Item::Weapon { type_index } => { type_index > u.weapon.unwrap_or(-1) && u.ammo[type_index as usize] > 0 as i32}
        Item::ShieldPotions { amount } => { u.shield_potions < constants.max_shield_potions_in_inventory }
        Item::Ammo { weapon_type_index, amount } => { true }
    }
}



