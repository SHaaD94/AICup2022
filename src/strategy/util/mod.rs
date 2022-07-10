use crate::model::{Obstacle, Vec2};
use crate::strategy::holder::get_constants;

//TODO find better solution
pub fn does_intersect(x1: f64, y1: f64, x2: f64, y2: f64, obstacles: &Vec<Obstacle>) -> bool {
    for obs in obstacles {
        let x0: f64 = obs.position.x;
        let y0: f64 = obs.position.y;

        let dist = ((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1)).abs() /
            ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
        if dist < obs.radius && !obs.can_shoot_through {
            return true;
        }
    }
    false
}