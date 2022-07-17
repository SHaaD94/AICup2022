use crate::model::{Item, Loot, Unit, Vec2};
use crate::strategy::holder::{get_constants, get_game, get_all_enemy_units, is_loot_booked};
use itertools::Itertools;
use libc::clone;
use std::any::Any;
use crate::strategy::behaviour::behaviour::{my_units_collision_score, my_units_magnet_score};

pub fn best_loot(unit: &Unit, loots: &Vec<Loot>, intersecting: bool) -> Option<Loot> {
    let constants = get_constants();
    let current_weapon = unit.weapon;
    let ammo = unit.ammo.clone();
    loots
        .iter()
        .filter(|l| !is_loot_booked(&l.id))
        .filter(|l| is_inside_zone(l))
        .filter(|l| is_loot_needed(l, unit))
        .filter(|l| {
            get_all_enemy_units()
                .iter()
                .find(|e| {
                    e.position.distance(&l.position) + get_constants().unit_radius
                        < e.firing_distance()
                })
                .is_none()
        })
        .filter(|l| {
            if !intersecting {
                unit.position.distance(&l.position) >= constants.unit_radius
            } else {
                unit.position.distance(&l.position) < constants.unit_radius
            }
        })
        .map(|l| {
            let score: i32 = match l.item {
                Item::Weapon { type_index } => {
                    if current_weapon.is_none() {
                        (type_index + 1) * 10
                    } else {
                        if ammo[type_index as usize] == 0 {
                            0
                        } else {
                            if current_weapon.unwrap() < type_index {
                                (type_index + 1) * 10
                            } else {
                                0
                            }
                        }
                    }
                }
                Item::ShieldPotions { amount } => {
                    if unit.shield_potions < constants.max_shield_potions_in_inventory {
                        (10.0 / (unit.health / constants.unit_health)).ceil() as i32
                    } else {
                        0
                    }
                }
                Item::Ammo {
                    weapon_type_index,
                    amount,
                } => {
                    let percent_of_max_ammo = ammo[weapon_type_index as usize] as f64
                        / constants.weapons[weapon_type_index as usize].max_inventory_ammo as f64;
                    if percent_of_max_ammo == 1.0 {
                        0
                    } else if percent_of_max_ammo == 0.0 {
                        (weapon_type_index + 1) * 5
                    } else {
                        ((weapon_type_index + 1) as f64 * 5.0 / percent_of_max_ammo).ceil() as i32
                    }
                }
            };
            // println!("{}, {}, {}", match l.item {
            //     Item::Weapon { .. } => { "w" }
            //     Item::ShieldPotions { .. } => { "shield" }
            //     Item::Ammo { .. } => { "ammo" }
            // }, score, -l.position.distance(&unit.position) + my_units_magnet_score(&l.position, unit));
            (score.clone() as f64
                 - l.position.distance(&unit.position)
                 - my_units_magnet_score(&l.position, unit), l)
        })
        .max_by(|(score1, _), (score2, _)| score1.partial_cmp(score2).unwrap())
        .map(|e| e.1.clone())
}

fn is_inside_zone(loot: &Loot) -> bool {
    let game = get_game();
    game.zone.current_center.distance(&loot.position) + 3.0 <= game.zone.current_radius
}

fn is_loot_needed(l: &Loot, unit: &Unit) -> bool {
    let constants = get_constants();
    let current_weapon = unit.weapon;
    let ammo = unit.ammo.clone();

    match l.item {
        Item::Weapon { type_index } => {
            if current_weapon.is_none() {
                true
            } else {
                if current_weapon.unwrap() < type_index {
                    true
                } else {
                    false
                }
            }
        }
        Item::ShieldPotions { amount } => {
            if unit.shield_potions < constants.max_shield_potions_in_inventory {
                true
            } else {
                false
            }
        }
        Item::Ammo {
            weapon_type_index,
            amount,
        } => {
            let percent_of_max_ammo = ammo[weapon_type_index as usize] as f64
                / constants.weapons[weapon_type_index as usize].max_inventory_ammo as f64;
            if percent_of_max_ammo == 1.0 {
                false
            } else if percent_of_max_ammo == 0.0 {
                true
            } else {
                true
            }
        }
    }
}