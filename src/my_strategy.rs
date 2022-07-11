use std::collections::HashMap;
use std::f64::consts::PI;
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
                for u in get_game().my_units() {
                    let default_view = get_constants().field_of_view;
                    let view_angle = u.weapon.map(|e|
                        default_view - (default_view - get_constants().weapons[e as usize].aim_field_of_view) * u.aim)
                        .unwrap_or(default_view) * PI / 180.0;

                    fn rotate(center: Vec2, angle: f64, distance: f64) -> Vec2 {
                        center + Vec2 { x: angle.cos() * distance, y: angle.sin() * distance }
                    }
                    // debug.add_poly_line(Vec::from([
                    //     u.position.clone(),
                    //     u.position.clone() + u.direction.clone(),
                    // ]), 0.3, BLUE.clone());
                    debug.add_poly_line(Vec::from([
                        u.position.clone(),
                        rotate(
                            u.position.clone(),
                            u.direction.angle(),
                            get_constants().view_distance),
                    ]), 0.1, GREEN.clone());
                    debug.add_poly_line(Vec::from([
                        u.position.clone(),
                        rotate(
                            u.position.clone(),
                            u.direction.angle() + view_angle / 2.0,
                            get_constants().view_distance),
                    ]), 0.1, BLUE.clone());
                    debug.add_poly_line(Vec::from([
                        u.position.clone(),
                        rotate(
                            u.position.clone(),
                            u.direction.angle() - view_angle / 2.0,
                            get_constants().view_distance),
                    ]), 0.1, RED.clone());
                    // let first = rotate(u.position.clone(), u.direction.clone(), view_angle / 2.0);
                    // let second = rotate(u.position.clone(), u.direction.clone(), -view_angle / 2.0);
                    // debug.add_segment(first, second, get_constants().view_distance, GREEN.clone());
                }
                // Self::draw_sounds(debug);
                Self::draw_units(debug);
                Self::draw_loot(debug);
                //draw projectiles
                for x in get_projectiles() {
                    debug.add_circle(x.position.clone(), 0.5, BLUE.clone())
                }
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