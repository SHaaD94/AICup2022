use itertools::Itertools;
use crate::model::{Constants, Game, Obstacle};

static mut GAME: Game = Game::const_default();
static mut CONSTANTS: Constants = Constants::const_default();
static mut NEAREST_OBSTACLES: Vec<(i32, Vec<Obstacle>)> = vec![];

pub fn get_constants() -> &'static Constants { unsafe { &CONSTANTS } }

pub fn get_game() -> &'static Game { unsafe { &GAME } }

pub fn set_constants(constants: Constants) { unsafe { CONSTANTS = constants } }

pub fn get_obstacles(unit_id: i32) -> Vec<Obstacle> {
    unsafe { &NEAREST_OBSTACLES }.iter().find(|(id, _)| id == &unit_id)
        .map(|(_, obstacles)| obstacles.clone()).unwrap_or(Vec::new())
}

pub fn set_game(game: Game) {
    let constants = get_constants();

    unsafe { NEAREST_OBSTACLES.clear(); }

    // set nearest obstacles
    game.my_units().iter().map(|u|
        (u.id.clone(), constants.obstacles.iter()
            .filter(|o| o.position.distance(&u.position) < constants.view_distance + o.radius)
            .map(|o| o.clone()).collect_vec())
    ).for_each(|x| unsafe { NEAREST_OBSTACLES.push(x) });

    unsafe { GAME = game }
}

