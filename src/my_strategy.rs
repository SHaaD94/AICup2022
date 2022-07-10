use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use itertools::Itertools;
use crate::debug_interface::DebugInterface;
use ai_cup_22::*;
use ai_cup_22::debugging::{BLUE, Color, GREEN, RED, TEAL};
use ai_cup_22::model::{Constants, Game, UnitOrder, Vec2};
use ai_cup_22::model::ActionOrder::Aim;
use ai_cup_22::strategy::get_order;
use ai_cup_22::strategy::holder::{get_constants, get_game, get_loot, get_obstacles, get_projectiles, get_units, set_constants, update_game};

pub struct MyStrategy {}

impl MyStrategy {
    pub fn new(constants: Constants) -> Self {
        set_constants(constants);
        MyStrategy {}
    }

    pub fn get_order(
        &mut self,
        game: Game,
        debug_interface: Option<&mut DebugInterface>,
    ) -> model::Order {
        update_game(game);

        match debug_interface {
            None => {}
            Some(debug) => {
                // Self::draw_sounds(debug);
                // Self::draw_units(debug);
                // Self::draw_loot(debug);
                //draw projectiles
                // for x in get_projectiles() {
                //     debug.add_circle(x.position.clone(), 0.5, BLUE.clone())
                // }
                // Self::draw_obstacles(debug)
            }
        }
        get_order()
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
            debug.add_circle(x.position.clone(), 1.5, RED.clone())
        }
    }

    fn draw_units(debug: &mut DebugInterface) {
        for x in get_units() {
            debug.add_circle(x.position.clone(), get_constants().unit_radius, BLUE.clone())
        }
    }

    pub fn debug_update(
        &mut self,
        displayed_tick: i32,
        debug_interface: &mut DebugInterface,
    ) {}
    pub fn finish(&mut self) {}
}