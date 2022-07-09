use std::collections::HashMap;
use itertools::Itertools;
use crate::debug_interface::DebugInterface;
use ai_cup_22::*;
use ai_cup_22::debugging::{Color, RED};
use ai_cup_22::model::{UnitOrder, Vec2};
use ai_cup_22::model::ActionOrder::Aim;
use ai_cup_22::strategy::get_order;

pub struct MyStrategy {
    constants: model::Constants,
}

impl MyStrategy {
    pub fn new(constants: model::Constants) -> Self {
        Self { constants }
    }

    pub fn get_order(
        &mut self,
        game: &model::Game,
        debug_interface: Option<&mut DebugInterface>,
    ) -> model::Order {
        match debug_interface {
            None => {}
            Some(debug) => {
                for x in &game.sounds {
                    debug.add_circle(x.position.clone(), 2.5, RED.clone())
                }
            }
        }
        get_order(game, &self.constants)
    }

    pub fn debug_update(
        &mut self,
        displayed_tick: i32,
        debug_interface: &mut DebugInterface,
    ) {}
    pub fn finish(&mut self) {}
}