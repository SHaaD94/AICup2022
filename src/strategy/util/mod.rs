use crate::debug_interface::DebugInterface;
use crate::model::{Obstacle, Projectile, Unit, Vec2};
use crate::strategy::holder::{get_constants, get_projectiles};
use itertools::Itertools;
use std::cmp::min;

pub fn rotate(center: Vec2, angle: f64, distance: f64) -> Vec2 {
    center
        + Vec2 {
            x: angle.cos() * distance,
            y: angle.sin() * distance,
        }
}

pub fn does_intersect_vec(v1: &Vec2, v2: &Vec2, obstacles: &Vec<Obstacle>) -> bool {
    does_intersect(v1.x, v1.y, v2.x, v2.y, obstacles)
}

pub fn does_intersect(x1: f64, y1: f64, x2: f64, y2: f64, obstacles: &Vec<Obstacle>) -> bool {
    for obs in obstacles.iter().filter(|o| {
        let min_x = if x1 < x2 { x1 } else { x2 } - o.radius;
        let max_x = if x1 >= x2 { x1 } else { x2 } + o.radius;
        let min_y = if y1 < y2 { y1 } else { y2 } - o.radius;
        let max_y = if y1 >= y2 { y1 } else { y2 } + o.radius;

        o.position.x >= min_x
            && o.position.x <= max_x
            && o.position.y >= min_y
            && o.position.y <= max_y
    }) {
        let x0: f64 = obs.position.x;
        let y0: f64 = obs.position.y;

        let dist = ((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1)).abs()
            / ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();

        if dist < obs.radius && !obs.can_shoot_through {
            return true;
        }
    }
    false
}

pub fn get_projectile_traces() -> Vec<Projectile> {
    let mut current_projectiles = get_projectiles().clone();
    let mut traces = Vec::new();
    while let Some(projectile) = current_projectiles.pop() {
        let next_pos = match projectile.position_after_ticks(1) {
            None => continue,
            Some(pos) => pos,
        };

        let next_projectile = Projectile {
            position: next_pos,
            life_time: (projectile.life_time_in_ticks() - 1.0) / get_constants().ticks_per_second,
            ..projectile.clone()
        };
        traces.push(next_projectile.clone());
        current_projectiles.push(next_projectile.clone());
    }
    traces
}

pub fn bullet_trace_score(bullets: &Vec<Projectile>, pos: &Vec2) -> f64 {
    bullets
        .iter()
        .filter(|b| b.position.distance(pos) <= get_constants().unit_radius + 0.1)
        .unique_by(|b| b.id)
        .map(|b| get_constants().weapons[b.weapon_type_index as usize].projectile_damage)
        .sum::<f64>()
        * 10000.0
}
