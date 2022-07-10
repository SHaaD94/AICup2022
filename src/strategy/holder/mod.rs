use std::collections::HashMap;
use itertools::Itertools;
use crate::model::{Constants, Game, Loot, Obstacle, Projectile, Unit};

static mut GAME: Game = Game::const_default();
static mut CONSTANTS: Constants = Constants::const_default();
static mut NEAREST_OBSTACLES: Vec<(i32, Vec<Obstacle>)> = vec![];

static mut LOOT_TO_TICK: Vec<(i32, Loot)> = vec![];
static mut LOOT: Vec<Loot> = vec![];

static mut UNIT_TO_TICK: Vec<(i32, Unit)> = vec![];
static mut UNITS: Vec<Unit> = vec![];

static mut PROJECTILES_TO_TICK: Vec<(i32, Projectile)> = vec![];
static mut PROJECTILES: Vec<Projectile> = vec![];

pub fn get_constants() -> &'static Constants { unsafe { &CONSTANTS } }

pub fn get_game() -> &'static Game { unsafe { &GAME } }

pub fn get_units() -> &'static Vec<Unit> { unsafe { &UNITS } }

pub fn set_constants(constants: Constants) { unsafe { CONSTANTS = constants } }

pub fn get_obstacles(unit_id: i32) -> Vec<Obstacle> {
    unsafe { &NEAREST_OBSTACLES }.iter().find(|(id, _)| id == &unit_id)
        .map(|(_, obstacles)| obstacles.clone()).unwrap_or(Vec::new())
}

pub fn update_game(game: Game) {
    let constants = get_constants();

    set_nearest_obstacles(&game, constants);
    update_units(&game);

    unsafe { GAME = game }
}

fn update_units(game: &Game) {
    let unit_ttl = 50;
    let mut units_hashmap = HashMap::new();
    for x in game.enemy_units() {
        units_hashmap.insert(x.id, (unit_ttl, x.clone()));
    }
    for x in unsafe { &UNIT_TO_TICK } {
        if !units_hashmap.contains_key(&x.1.id) {
            if x.0 - 1 > 0 {
                units_hashmap.insert(x.1.id, (x.0 - 1, x.1.clone()));
            }
        }
    }
    unsafe { UNIT_TO_TICK = units_hashmap.iter().map(|e| e.1.clone()).collect_vec() };
    unsafe { UNITS = units_hashmap.iter().map(|e| e.1.1.clone()).collect_vec() };
}

fn set_nearest_obstacles(game: &Game, constants: &Constants) {
    unsafe { NEAREST_OBSTACLES.clear(); }
    game.my_units().iter().map(|u|
        (u.id.clone(), constants.obstacles.iter()
            .filter(|o| o.position.distance(&u.position) < constants.view_distance + o.radius)
            .map(|o| o.clone()).collect_vec())
    ).for_each(|x| unsafe { NEAREST_OBSTACLES.push(x) });
}

