use crate::debug_interface::DebugInterface;
use ai_cup_22::debugging::{Color, BLUE, GREEN, RED, TEAL, TRANSPARENT_GREEN, YELLOW};
use ai_cup_22::model::ActionOrder::Aim;
use ai_cup_22::model::{Constants, Game, UnitOrder, Vec2};
use ai_cup_22::strategy::get_order;
use ai_cup_22::strategy::holder::{
    get_constants, get_game, get_loot, get_obstacles, get_projectiles, get_all_enemy_units, set_constants,
    update_game,
};
use ai_cup_22::strategy::util::get_projectile_traces;
use ai_cup_22::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::atomic::AtomicPtr;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MyStrategy {}

impl MyStrategy {
    pub fn new(constants: Constants) -> Self {
        set_constants(constants);
        MyStrategy {}
    }

    pub fn get_order(
        &mut self,
        game: Game,
        debug_interface: &mut Option<&mut DebugInterface>,
    ) -> model::Order {
        // let start = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .expect("Time went backwards");
        update_game(game, debug_interface);

        if let Some(debug) = debug_interface.as_mut() {
            // Self::draw_sounds(debug);
            // Self::draw_vision(debug);
            Self::draw_units(debug);
            Self::draw_points_around(debug);
            Self::draw_projectile_traces(debug)
            // Self::draw_loot(debug);
            // Self::draw_projectiles(debug)
            // Self::draw_obstacles(debug)
        }
        // println!("{}", SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .expect("Time went backwards").as_nanos() - start.as_nanos());
        get_order(debug_interface)
    }

    fn draw_vision(debug: &mut DebugInterface) {
        for u in get_game().my_units() {
            let (left_angle, right_angle) = u.view_segment_angles();

            debug.add_pie(
                u.position.clone(),
                get_constants().view_distance,
                left_angle,
                right_angle,
                TRANSPARENT_GREEN.clone(),
            );
        }
    }
    fn draw_projectile_traces(debug: &mut DebugInterface) {
        for x in get_projectile_traces() {
            debug.add_circle(x.position, 0.1, BLUE.clone());
        }
    }

    fn draw_points_around(debug: &mut DebugInterface) {
        for unit in get_game().my_units() {
            for x in unit.points_around_unit(false) {
                debug.add_circle(x, 0.1, GREEN.clone());
            }
        }
    }

    fn draw_projectiles(debug: &mut DebugInterface) {
        for x in get_projectiles() {
            debug.add_circle(x.position.clone(), 0.5, BLUE.clone())
        }
    }

    fn draw_obstacles(debug: &mut DebugInterface) {
        for unit in get_game().my_units() {
            for obstacle in get_obstacles(unit.id) {
                debug.add_circle(obstacle.position.clone(), obstacle.radius, GREEN.clone());
            }
        }
    }

    fn draw_loot(debug: &mut DebugInterface) {
        for x in get_loot() {
            debug.add_circle(x.position.clone(), 0.5, TEAL.clone())
        }
    }

    fn draw_sounds(debug: &mut DebugInterface) {
        for x in &get_game().sounds {
            debug.add_circle(
                x.position.clone(),
                0.5 * ((x.type_index + 1) as f64),
                YELLOW.clone(),
            )
        }
    }

    fn draw_units(debug: &mut DebugInterface) {
        for x in get_all_enemy_units() {
            debug.add_circle(
                x.position.clone(),
                get_constants().unit_radius,
                BLUE.clone(),
            )
        }
    }

    pub fn debug_update(&mut self, displayed_tick: i32, debug_interface: &mut DebugInterface) {}
    pub fn finish(&mut self) {}
}
