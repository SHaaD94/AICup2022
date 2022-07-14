use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, GREEN, RED, TRANSPARENT_BLUE, TRANSPARENT_GREEN};
use crate::model::{Unit, UnitOrder, Vec2};
use crate::model::ActionOrder::Pickup;
use crate::strategy::behaviour::behaviour::Behaviour;
use crate::strategy::holder::{get_constants, get_game, get_loot, remove_loot};
use crate::strategy::loot::best_loot;
use crate::strategy::util::{bullet_trace_score, get_projectile_traces};

pub struct MoveToCenterOrLoot {}

impl Behaviour for MoveToCenterOrLoot {
    fn should_use(&self, unit: &Unit) -> bool { true }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        if let Some(debug) = debug_interface.as_mut() {
            debug.add_placed_text(
                unit.position.clone() - Vec2 { x: 0.0, y: -5.0 },
                "Move".to_owned(),
                Vec2 { x: 1.0, y: 1.0 },
                1.0,
                RED.clone(),
            )
        }

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
        let traces = get_projectile_traces();

        let goal = best_not_intersecting_loot.map(|l| l.position).unwrap_or(next_zone_center);
        let result_move = unit.points_around_unit().iter()
            .map(|e| (e, bullet_trace_score(&traces, &e) + e.distance(&goal)))
            .min_by(|e1, e2| {
                f64::partial_cmp(&e1.1, &e2.1).unwrap()
            }).unwrap().0.clone();
        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(result_move.clone(), 0.1, BLUE.clone());
            debug.add_circle(goal.clone(), 1.0, TRANSPARENT_BLUE.clone());
        }
        let rotation = if get_game().current_tick % 100 >= 85 {
            Vec2 { x: -unit.direction.y, y: unit.direction.x }
        } else {
            (goal.clone() - unit.position.clone())
        };

        UnitOrder {
            target_velocity: (result_move.clone() - unit.position.clone()) * 1000.0,
            target_direction: rotation,
            // target_direction: move_target.clone() - unit.position.clone(),
            action: best_intersecting_loot.map(|l| Pickup { loot: l.id }),
        }
    }
}
