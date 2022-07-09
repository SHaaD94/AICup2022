use crate::model::{Constants, Game};

static mut GAME: Game = Game::const_default();
static mut CONSTANTS: Constants = Constants::const_default();

pub fn get_constants() -> &'static Constants { unsafe { &CONSTANTS } }

pub fn get_game() -> &'static Game { unsafe { &GAME } }

pub fn set_constants(constants: Constants) { unsafe { CONSTANTS = constants } }

pub fn set_game(game: Game) { unsafe { GAME = game } }

