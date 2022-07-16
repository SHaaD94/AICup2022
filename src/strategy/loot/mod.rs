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
    let score2loot = loots
        .iter()
        .filter(|l| !is_loot_booked(&l.id))
        .filter(|l| is_inside_zone(l))
        // .filter(|l| match unit.my_closest_other_unit() {
        //     None => { true }
        //     Some(other) => other.position.distance(&l.position) < 30.0 || unit.position.distance(&l.position) < 10.0
        // })
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
                        (type_index + 1) * 100
                    } else {
                        if ammo[type_index as usize] == 0 {
                            0
                        } else {
                            if current_weapon.unwrap() < type_index {
                                (type_index + 1) * 100
                            } else {
                                0
                            }
                        }
                    }
                }
                Item::ShieldPotions { amount } => {
                    if unit.shield_potions < constants.max_shield_potions_in_inventory {
                        (300.0 / (unit.health / constants.unit_health)).ceil() as i32
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
                        (weapon_type_index + 1) * 75
                    } else {
                        ((weapon_type_index + 1) as f64 * 75.0 / percent_of_max_ammo).ceil() as i32
                    }
                }
            };
            (score.clone(), l)
        })
        .filter(|e| e.0 > 0)
        .collect_vec();
    let max = score2loot.iter().max_by_key(|e| e.0);
    max.map(|(score, _)| {
        score2loot
            .iter()
            .filter(|e| &e.0 == score)
            .map(|e| e.1)
            .min_by(|a, b| {
                fn score(p: &Vec2, unit: &Unit) -> f64 {
                    p.distance(&unit.position)
                        - my_units_magnet_score(&p, unit)
                }
                score(&a.position, unit).partial_cmp(&score(&b.position, unit)).unwrap()
            })
            .map(|e| e.clone())
    })
        .flatten()
}

fn is_inside_zone(loot: &Loot) -> bool {
    let game = get_game();
    game.zone.current_center.distance(&loot.position) + 3.0 <= game.zone.current_radius
}
