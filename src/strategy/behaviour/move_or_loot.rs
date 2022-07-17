use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, GREEN, RED, TRANSPARENT_BLUE, TRANSPARENT_GREEN};
use crate::model::ActionOrder::Pickup;
use crate::model::{Loot, Unit, UnitOrder, Vec2};
use crate::strategy::behaviour::behaviour::{my_units_collision_score, write_behaviour, Behaviour};
use crate::strategy::holder::{book_loot, get_constants, get_game, get_loot, remove_loot};
use crate::strategy::loot::best_loot;
use crate::strategy::util::{bullet_trace_score, get_projectile_traces, rotate};

pub struct MoveOrLoot {}

impl Behaviour for MoveOrLoot {
    fn should_use(&self, unit: &Unit) -> bool {
        true
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        write_behaviour(unit, "Move".to_owned(), debug_interface);

        let game = get_game();
        let constants = get_constants();
        let loot = get_loot();
        let best_not_intersecting_loot = best_loot(unit, loot, false);
        let best_intersecting_loot = best_loot(unit, loot, true);
        let can_pickup = unit.aim == 0.0 && unit.action.is_none();
        if let Some(loot) = &best_intersecting_loot {
            if can_pickup {
                unsafe {
                    remove_loot(loot.id.clone());
                }
            }
        }
        if let Some(ref l) = best_intersecting_loot {
            book_loot(l.id.clone());
        }
        if let Some(ref l) = best_not_intersecting_loot {
            book_loot(l.id.clone());
        }

        let traces = get_projectile_traces();

        let goal = match best_not_intersecting_loot {
            None => {
                let other = unit.my_closest_other_unit();
                if other.is_some() && other.unwrap().0 > 10.0 {
                    other.unwrap().1.position
                } else {
                    let angle = (unit.position.clone() - game.zone.current_center.clone()).angle();
                    rotate(
                        game.zone.current_center.clone(),
                        angle + 0.1,
                        game.zone.current_radius * 0.85,
                    )
                }
            }
            Some(ref g) => g.position,
        };

        let result_move = unit
            .points_around_unit(true)
            .iter()
            .map(|p| {
                (
                    p,
                    bullet_trace_score(&traces, &p)
                        + my_units_collision_score(p, unit)
                        + p.distance(&goal),
                )
            })
            .min_by(|e1, e2| f64::partial_cmp(&e1.1, &e2.1).unwrap())
            .unwrap()
            .0
            .clone();
        if let Some(debug) = debug_interface.as_mut() {
            debug.add_circle(result_move.clone(), 0.1, BLUE.clone());
            debug.add_circle(goal.clone(), 1.0, TRANSPARENT_BLUE.clone());
        }
        let rotation = if get_game().current_tick % 100 >= 85 {
            Vec2 {
                x: -unit.direction.y,
                y: unit.direction.x,
            }
        } else {
            (goal.clone() - unit.position.clone())
        };

        let pickup_action = if can_pickup {
            best_intersecting_loot.map(|l| Pickup { loot: l.id })
        } else {
            None
        };
        UnitOrder {
            target_velocity: (result_move.clone() - unit.position.clone()) * 1000.0,
            target_direction: rotation,
            // target_direction: move_target.clone() - unit.position.clone(),
            action: pickup_action,
        }
    }
}
