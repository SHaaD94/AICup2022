use libc::clone;
use crate::model::{Item, Loot, Unit};
use crate::strategy::holder::{get_constants, get_game};

pub fn best_loot(unit: &Unit, loots: Vec<&Loot>, intersecting: bool) -> Option<Loot> {
    let constants = get_constants();
    let current_weapon = unit.weapon;
    let ammo = unit.ammo.clone();
    loots.iter()
        .filter(|l| is_inside_zone(l))
        .filter(|l| if !intersecting { unit.position.distance(&l.position) >= constants.unit_radius } else { unit.position.distance(&l.position) < constants.unit_radius })
        .map(|l| {
            let score: i32 = match l.item {
                Item::Weapon { type_index } => {
                    if current_weapon.is_none() {
                        type_index * 100
                    } else {
                        if ammo[type_index as usize] == 0 { 0 } else {
                            if current_weapon.unwrap() < type_index {
                                type_index * 100
                            } else {
                                0
                            }
                        }
                    }
                }
                Item::ShieldPotions { amount } => {
                    if unit.shield_potions < constants.max_shield_potions_in_inventory {
                        (75.0 / unit.health / constants.unit_health).ceil() as i32
                    } else {
                        0
                    }
                }
                Item::Ammo { weapon_type_index, amount } => {
                    let percent_of_max_ammo = ammo[weapon_type_index as usize] as f64 * constants.weapons[weapon_type_index as usize].max_inventory_ammo as f64;
                    if percent_of_max_ammo == 0.0 {
                        weapon_type_index * 75
                    } else {
                        (weapon_type_index as f64 * 75.0 / percent_of_max_ammo).ceil() as i32
                    }
                }
            };
            (score.clone(), *l)
        })
        .filter(|e| e.0 > 0)
        .max_by_key(|e| e.0)
        .map(|(score, l)| l.clone())
}

fn is_inside_zone(loot: &Loot) -> bool {
    let game = get_game();
    game.zone.current_center.distance(&loot.position) <= game.zone.current_radius
}
