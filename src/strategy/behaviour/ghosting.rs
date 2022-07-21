use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, GREEN, RED, TRANSPARENT_BLUE, TRANSPARENT_GREEN};
use crate::model::ActionOrder::Pickup;
use crate::model::{Game, Loot, Unit, UnitOrder, Vec2};
use crate::strategy::behaviour::behaviour::{
    my_units_collision_score, write_behaviour, zone_penalty, Behaviour,
};
use crate::strategy::holder::{book_loot, get_constants, get_game, get_loot, remove_loot};
use crate::strategy::loot::best_loot;
use crate::strategy::util::{bullet_trace_score, get_projectile_traces, rotate};

pub struct Ghosting {}

impl Behaviour for Ghosting {
    fn should_use(&self, unit: &Unit) -> bool {
        unit.remaining_spawn_time.is_some()
    }

    fn order(&self, unit: &Unit, debug_interface: &mut Option<&mut DebugInterface>) -> UnitOrder {
        write_behaviour(unit, "Ghosting".to_owned(), debug_interface);

        let game = get_game();

        let goal = if game.current_tick
            < (get_constants().spawn_time * get_constants().ticks_per_second).ceil() as i32
        {
            loot_or_near_the_zone(unit, game)
        } else {
            match unit.my_closest_other_unit() {
                None => loot_or_near_the_zone(unit, game),
                Some(u) => u.1.position,
            }
        };

        let result_move = unit
            .points_around_unit(false)
            .iter()
            .map(|p| {
                (
                    p,
                    p.distance(&goal) + my_units_collision_score(&p, unit) + zone_penalty(p),
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

        UnitOrder {
            target_velocity: (result_move.clone() - unit.position.clone()) * 1000.0,
            //constantly rotate
            target_direction: Vec2 {
                x: -unit.direction.y,
                y: unit.direction.x,
            },
            action: None,
        }
    }
}

fn loot_or_near_the_zone(unit: &Unit, game: &Game) -> Vec2 {
    let best_not_intersecting_loot = best_loot(unit, get_loot(), false);
    match best_not_intersecting_loot {
        None => rotate(
            game.zone.current_center.clone(),
            (unit.position.clone() - game.zone.current_center.clone()).angle() + 0.1,
            game.zone.current_radius * 0.85,
        ),
        Some(loot) => {
            book_loot(loot.id);
            loot.position
        }
    }
}
