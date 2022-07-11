use std::cmp::min;
use crate::model::{Unit, UnitOrder};
use crate::model::ActionOrder::Aim;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::holder::{get_constants, get_game, get_obstacles, get_units};
use crate::strategy::util::does_intersect;

pub struct Fighting {}

impl Behaviour for Fighting {
    fn should_use(&self, unit: &Unit) -> bool {
        let have_weapon_and_ammo = match unit.weapon {
            None => { false }
            Some(weapon) => { unit.ammo[weapon as usize] != 0 }
        };
        if !have_weapon_and_ammo { return false; };

        get_units().iter().filter(|e| simulation(unit, e)).find(|e|
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

        UnitOrder {
            target_velocity: if simulation(unit, target) { vector } else { vector * -1.0 },
            target_direction: target.position.clone() - unit.position.clone(),
            action: Some(Aim {
                shoot: !intersects_with_obstacles
            }),
        }
    }
}

fn simulation(u1: &Unit, u2: &Unit) -> bool {
    if u1.weapon.is_none() { return false; }
    if u2.weapon.is_none() { return true; }
    let constants = get_constants();
    let mut ammo1 = u1.ammo[u1.weapon.unwrap() as usize];
    let w1 = &constants.weapons[u1.weapon.unwrap() as usize];
    let mut h1 = u1.health;
    let mut ammo2 = u2.ammo[u2.weapon.unwrap() as usize];
    let w2 = &constants.weapons[u2.weapon.unwrap() as usize];
    let mut h2 = u2.health;

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
            tick1 += w1.get_fire_rate_in_ticks();
            h2 -= w1.projectile_damage;
            ammo1 -= 1;
        }
        if tick2 == 0 {
            tick2 += w2.get_fire_rate_in_ticks();
            h1 -= w2.projectile_damage;
            ammo2 -= 1;
        }
    }
    return h1 >= 0.0 && h2 <= 0.0;
}
