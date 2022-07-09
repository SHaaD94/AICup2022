use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use itertools::Itertools;
use crate::debug_interface::DebugInterface;
use ai_cup_22::*;
use ai_cup_22::debugging::{Color, RED};
use ai_cup_22::model::{Constants, Game, UnitOrder, Vec2};
use ai_cup_22::model::ActionOrder::Aim;
use ai_cup_22::strategy::get_order;
use ai_cup_22::strategy::holder::{get_game, set_constants, set_game};

pub struct MyStrategy {}

impl MyStrategy {
    pub fn new(constants: Constants) -> Self {
        set_constants(constants);
        MyStrategy {}
    }

    pub fn get_order(
        &mut self,
        game: &Game,
        debug_interface: Option<&mut DebugInterface>,
    ) -> model::Order {
        set_game(game.clone());

        match debug_interface {
            None => {}
            Some(debug) => {
                for x in &get_game().sounds {
                    debug.add_circle(x.position.clone(), 2.5, RED.clone())
                }
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