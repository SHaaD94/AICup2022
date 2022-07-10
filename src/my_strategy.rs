use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use itertools::Itertools;
use crate::debug_interface::DebugInterface;
use ai_cup_22::*;
use ai_cup_22::debugging::{BLUE, Color, GREEN, RED};
use ai_cup_22::model::{Constants, Game, UnitOrder, Vec2};
use ai_cup_22::model::ActionOrder::Aim;
use ai_cup_22::strategy::get_order;
use ai_cup_22::strategy::holder::{get_constants, get_game, get_obstacles, get_units, set_constants, update_game};

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
                // draw sounds
                for x in &get_game().sounds {
                    debug.add_circle(x.position.clone(), 1.5, RED.clone())
                }
                // draw units
                for x in get_units() {
                    debug.add_circle(x.position.clone(), get_constants().unit_radius, BLUE.clone())
                }
                // draw obstacles
                // for unit in game.my_units() {
                //     for obstacle in get_obstacles(unit.id) {
                //         debug.add_circle(obstacle.position.clone(), obstacle.radius, GREEN.clone())
                //     }
                // }
            }
        }
        get_order()
    }

    pub fn debug_update(
        &mut self,
        displayed_tick: i32,
        debug_interface: &mut DebugInterface,
    ) {}
    pub fn finish(&mut self) {}
}