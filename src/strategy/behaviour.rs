use std::cmp::min;
use itertools::Itertools;
use crate::model::{ActionOrder, Loot, Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::{Aim, Pickup, UseShieldPotion};
use crate::model::Item::ShieldPotions;
use crate::strategy::holder::{get_constants, get_game, get_loot, get_obstacles, get_units, remove_loot};
use crate::strategy::loot::best_loot;
use crate::strategy::util::does_intersect;

pub trait Behaviour: Sync {
    fn should_use(&self, unit: &Unit) -> bool;
    fn order(&self, unit: &Unit) -> UnitOrder;
}

pub struct UseHeal {}

impl Behaviour for UseHeal {
    fn should_use(&self, unit: &Unit) -> bool {
        unit.shield < get_constants().max_shield && unit.shield_potions > 0
    }

    fn order(&self, unit: &Unit) -> UnitOrder {
        UnitOrder {
            target_velocity: unit.velocity.clone(),
            target_direction: unit.direction.clone(),
            action: Some(UseShieldPotion {}),
        }
    }
}

pub struct MoveToCenterOrLoot {}

impl Behaviour for MoveToCenterOrLoot {
    fn should_use(&self, unit: &Unit) -> bool { true }

    fn order(&self, unit: &Unit) -> UnitOrder {
        let game = get_game();
        let constants = get_constants();
        let loot = get_loot();
        let best_not_intersecting_loot = best_loot(unit, loot, false);
        let next_zone_center = Vec2 {
            x: game.zone.next_center.x,
            y: game.zone.next_center.y,
        };
        let best_intersecting_loot = best_loot(unit, loot, true);
        if let Some(loot) = &best_intersecting_loot {
            unsafe { remove_loot(loot.id.clone()); }
        }
        UnitOrder {
            target_velocity: (best_not_intersecting_loot.as_ref().map(|l| l.position.clone()).unwrap_or(next_zone_center.clone()) - unit.position.clone()) * 1000.0,
            target_direction: best_not_intersecting_loot.map(|l| l.position).unwrap_or(next_zone_center) - unit.position.clone(),
            action: best_intersecting_loot.map(|l| Pickup { loot: l.id }),
        }
    }
}

pub struct Fighting {}

impl Behaviour for Fighting {
    fn should_use(&self, unit: &Unit) -> bool {
        let have_weapon_and_ammo = match unit.weapon {
            None => { false }
            Some(weapon) => { unit.ammo[weapon as usize] != 0 }
        };
        if !have_weapon_and_ammo { return false; };

        get_units().iter().find(|e|
            e.position.distance(&unit.position) < get_constants().weapons[unit.weapon.unwrap() as usize].firing_distance()
        ).is_some()
    }

    fn order(&self, unit: &Unit) -> UnitOrder {
        let game = get_game();
        let constants = get_constants();

        let target = get_units().iter().min_by_key(|e| e.position.distance(&unit.position) as i64).unwrap();

        let intersects_with_obstacles = does_intersect(
            unit.position.x,
            unit.position.y,
            target.position.x,
            target.position.y,
            &get_obstacles(unit.id),
        );
        let vector = target.position.clone() - unit.position.clone();
        fn simulation(u1: &Unit, u2: &Unit) -> bool {
            if u1.weapon.is_none() { return false; }
            if u2.weapon.is_none() { return true; }
            let constants = get_constants();
            let mut ammo1 = u1.ammo[u1.weapon.unwrap() as usize];
            let w1 = &constants.weapons[u1.weapon.unwrap() as usize];
            let mut h1 = u1.health;
            let mut ammo2 = u2.ammo[u2.weapon.unwrap() as usize];
            let w2 = u2.ammo[u2.weapon.unwrap() as usize];
            let mut h2 = u2.health;
            let w2 = &constants.weapons[u2.weapon.unwrap() as usize];

            let mut tick1 = 0;
            let mut tick2 = 0;
            while h1 >= 0.0 && h2 >= 0.0 {
                if ammo1 == 0 {
                    h1 = 0.0;
                    break;
                }
                if ammo2 == 0 {
                    h2 = 0.0;
                    break;
                }
                let min = min(tick1, tick2);
                tick1 -= min;
                tick2 -= min;
                if tick1 == 0 {
                    tick1 += (w1.rounds_per_second / constants.ticks_per_second).ceil() as i32;
                    h2 -= w1.projectile_damage;
                    ammo1 -= 1;
                }
                if tick2 == 0 {
                    tick2 += (w2.rounds_per_second / constants.ticks_per_second).ceil() as i32;
                    h1 -= w2.projectile_damage;
                    ammo2 -= 1;
                }
            }
            return h1 > 0.0;
        }
        UnitOrder {
            target_velocity: if simulation(unit, target) { vector } else { vector * -1.0 },
            target_direction: target.position.clone() - unit.position.clone(),
            action: Some(Aim {
                shoot: !intersects_with_obstacles
            }),
        }
    }
}
