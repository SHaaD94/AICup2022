
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
