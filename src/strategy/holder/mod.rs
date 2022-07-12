use std::collections::HashMap;
use itertools::Itertools;
use crate::debug_interface::DebugInterface;
use crate::debugging::{BLUE, RED};
use crate::model::{Constants, Game, Loot, Obstacle, Projectile, Unit, Vec2};

static mut GAME: Game = Game::const_default();
static mut CONSTANTS: Constants = Constants::const_default();
static mut NEAREST_OBSTACLES: Vec<(i32, Vec<Obstacle>)> = vec![];

static mut LOOT_TO_TICK: Vec<(i32, Loot)> = vec![];
static mut LOOT: Vec<Loot> = vec![];

static mut UNIT_TO_TICK: Vec<(i32, Unit)> = vec![];
static mut UNITS: Vec<Unit> = vec![];

static mut PROJECTILES: Vec<Projectile> = vec![];

pub fn get_constants() -> &'static Constants { unsafe { &CONSTANTS } }

pub fn get_game() -> &'static Game { unsafe { &GAME } }

pub fn get_units() -> &'static Vec<Unit> { unsafe { &UNITS } }

pub fn get_loot() -> &'static Vec<Loot> { unsafe { &LOOT } }

pub unsafe fn remove_loot(id_to_remove: i32) {
    let mut res = Vec::new();
    for x in &LOOT_TO_TICK {
        if x.1.id != id_to_remove {
            res.push((x.clone()));
        }
    }
    LOOT_TO_TICK = res;
    unsafe { LOOT = LOOT_TO_TICK.iter().map(|e| e.1.clone()).collect_vec() };
}

pub fn get_projectiles() -> &'static Vec<Projectile> { unsafe { &PROJECTILES } }

pub fn set_constants(constants: Constants) { unsafe { CONSTANTS = constants } }

pub fn get_obstacles(unit_id: i32) -> Vec<Obstacle> {
    unsafe { &NEAREST_OBSTACLES }.iter().find(|(id, _)| id == &unit_id)
        .map(|(_, obstacles)| obstacles.clone()).unwrap_or(Vec::new())
}

pub fn update_game(game: Game, debug_interface: &mut Option<&mut DebugInterface>) {
    let constants = get_constants();

    set_nearest_obstacles(&game, constants);
    update_units(&game, debug_interface);
    update_loot(&game);
    update_projectiles(&game);

    unsafe { GAME = game }
}

fn update_units(game: &Game, debug_interface: &mut Option<&mut DebugInterface>) {
    let unit_ttl = 50;
    let mut units_hashmap = HashMap::new();
    for x in game.enemy_units() {
        units_hashmap.insert(x.id, (unit_ttl, x.clone()));
    }
    for x in unsafe { &UNIT_TO_TICK } {
        if !units_hashmap.contains_key(&x.1.id) && !inside_vision(game, &x.1.position) {
            if x.0 - 1 > 0 {
                units_hashmap.insert(x.1.id, (x.0 - 1, x.1.clone()));
            }
        }
    }
    //deduce shooter position by projectiles
    for projectile in &game.projectiles {
        let w_id = projectile.weapon_type_index as usize;
        let w = &get_constants().weapons[w_id];
        let fly_time = w.projectile_life_time - projectile.life_time;

        let unit_pos = projectile.position.clone() - (projectile.velocity.clone() * fly_time);
        let ticks = (fly_time * &get_constants().ticks_per_second).ceil() as i32;

        // let unit_is_absent_or_new_data_is_fresher = units_hashmap.get(&projectile.shooter_id)
            // .map(|e| e.0 < unit_ttl - ticks).unwrap_or(true);

        println!("{}",ticks);

        if projectile.shooter_player_id != game.my_id
            && (!units_hashmap.contains_key(&projectile.shooter_id) || units_hashmap.get(&projectile.shooter_id).unwrap().0 < unit_ttl - ticks)
            && !inside_vision(game, &unit_pos) {

            // if let Some(d) = debug_interface.as_mut() {
            //     d.add_circle(unit_pos.clone(), 15.0, RED.clone());
            //     d.add_circle(projectile.position.clone(), 5.0, BLUE.clone());
            // }

            let imaginary_unit = Unit {
                id: projectile.shooter_id,
                position: unit_pos,
                direction: projectile.velocity.clone(),
                weapon: Some(w_id as i32),
                health: get_constants().unit_health.clone(),
                ammo: Vec::from([10, 10, 10, 10]),
                ..Unit::default()
            };
            units_hashmap.insert(projectile.shooter_id, (unit_ttl - ticks, imaginary_unit));
        }
    }

    unsafe { UNIT_TO_TICK = units_hashmap.iter().map(|e| e.1.clone()).collect_vec() };
    unsafe { UNITS = units_hashmap.iter().map(|e| e.1.1.clone()).collect_vec() };
}

fn update_loot(game: &Game) {
    let loot_ttl = 150;
    let mut loot_hashmap = HashMap::new();
    for x in &game.loot {
        loot_hashmap.insert(x.id, (loot_ttl, x.clone()));
    }
    for x in unsafe { &LOOT_TO_TICK } {
        if !loot_hashmap.contains_key(&x.1.id) && !inside_vision(game, &x.1.position) {
            if x.0 - 1 > 0 {
                loot_hashmap.insert(x.1.id, (x.0 - 1, x.1.clone()));
            }
        }
    }
    unsafe { LOOT_TO_TICK = loot_hashmap.iter().map(|e| e.1.clone()).collect_vec() };
    unsafe { LOOT = loot_hashmap.iter().map(|e| e.1.1.clone()).collect_vec() };
}

fn update_projectiles(game: &Game) {
    let ticks_per_second = get_constants().ticks_per_second;
    let mut projectiles_map = HashMap::new();
    for x in &game.projectiles {
        projectiles_map.insert(x.id, x.clone());
    }
    for x in unsafe { &PROJECTILES } {
        if !projectiles_map.contains_key(&x.id) {
            let life_time_after = (x.life_time_in_ticks() - 1.0) / ticks_per_second;
            let new_pos = x.position_after_ticks(1);
            let intersects_with_units = get_units().iter()
                .find(|e| e.position.distance(&new_pos) < get_constants().unit_radius).is_some();
            let intersects_with_obstacles = game.my_units().iter().map(|u| get_obstacles(u.id)).flatten()
                .filter(|e| !e.can_shoot_through)
                .map(|e| e.position)
                .find(|e| e.distance(&new_pos) < get_constants().unit_radius).is_some();

            if life_time_after > 0.0 && !intersects_with_units && !intersects_with_obstacles
                && !inside_vision(game, &x.position) {
                projectiles_map.insert(x.id, Projectile {
                    life_time: life_time_after,
                    id: x.id,
                    weapon_type_index: x.weapon_type_index,
                    shooter_id: x.shooter_id,
                    shooter_player_id: x.shooter_player_id,
                    position: new_pos,
                    velocity: x.velocity.clone(),
                });
            }
        }
    }
    unsafe { PROJECTILES = projectiles_map.iter().map(|e| e.1.clone()).collect_vec() };
}

fn inside_vision(game: &Game, x: &Vec2) -> bool {
    game.my_units().iter()
        .filter(|e| e.position.distance(x) <= get_constants().view_distance)
        .find(|u| {
            let (left_angle, right_angle) = u.view_segment_angles();
            let angle = (x.clone() - u.position.clone()).angle();
            (left_angle >= angle && right_angle <= angle) || (left_angle <= angle && right_angle >= angle)
        }).is_some()
}

fn set_nearest_obstacles(game: &Game, constants: &Constants) {
    unsafe { NEAREST_OBSTACLES.clear(); }
    game.my_units().iter().map(|u|
        (u.id.clone(), constants.obstacles.iter()
            .filter(|o| o.position.distance(&u.position) < constants.view_distance + o.radius)
            .map(|o| o.clone()).collect_vec())
    ).for_each(|x| unsafe { NEAREST_OBSTACLES.push(x) });
}

