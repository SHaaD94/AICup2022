use itertools::Itertools;
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::{Aim, Pickup};
use crate::strategy::holder::{get_constants, get_game, get_loot, get_obstacles, get_units};
use crate::strategy::loot::best_loot;
use crate::strategy::util::does_intersect;

pub trait Behaviour: Sync {
    fn should_use(&self, unit: &Unit) -> bool;
    fn order(&self, unit: &Unit) -> UnitOrder;
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
        println!("{}", intersects_with_obstacles);
        UnitOrder {
            target_velocity: target.position.clone() - unit.position.clone(),
            target_direction: target.position.clone() - unit.position.clone(),
            action: Some(Aim {
                shoot: !intersects_with_obstacles
            }),
        }
    }
}
