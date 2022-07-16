use super::*;
use crate::strategy::holder::{get_constants, get_game, get_obstacles};
use crate::strategy::util::rotate;
use std::f64::consts::PI;
use itertools::Itertools;

/// A unit
#[derive(Clone, Debug)]
pub struct Unit {
    /// Unique id
    pub id: i32,
    /// Id of the player (team) controlling the unit
    pub player_id: i32,
    /// Current health
    pub health: f64,
    /// Current shield value
    pub shield: f64,
    /// Left extra lives of this unit
    pub extra_lives: i32,
    /// Current position of unit's center
    pub position: model::Vec2,
    /// Remaining time until unit will be spawned, or None
    pub remaining_spawn_time: Option<f64>,
    /// Current velocity
    pub velocity: model::Vec2,
    /// Current view direction (vector of length 1)
    pub direction: model::Vec2,
    /// Value describing process of aiming (0 - not aiming, 1 - ready to shoot)
    pub aim: f64,
    /// Current action unit is performing, or None
    pub action: Option<model::Action>,
    /// Tick when health regeneration will start (can be less than current game tick)
    pub health_regeneration_start_tick: i32,
    /// Index of the weapon this unit is holding (starting with 0), or None
    pub weapon: Option<i32>,
    /// Next tick when unit can shoot again (can be less than current game tick)
    pub next_shot_tick: i32,
    /// List of ammo in unit's inventory for every weapon type
    pub ammo: Vec<i32>,
    /// Number of shield potions in inventory
    pub shield_potions: i32,
}

impl Unit {
    pub fn default() -> Unit {
        Unit {
            id: 0,
            player_id: 0,
            health: 0.0,
            shield: 0.0,
            extra_lives: 0,
            position: Default::default(),
            remaining_spawn_time: None,
            velocity: Default::default(),
            direction: Default::default(),
            aim: 0.0,
            action: None,
            health_regeneration_start_tick: 0,
            weapon: None,
            next_shot_tick: 0,
            ammo: vec![],
            shield_potions: 0,
        }
    }

    pub fn ammo_for_current_weapon(&self) -> i32 {
        match self.weapon {
            None => { 0 }
            Some(w) => {
                self.ammo[w as usize]
            }
        }
    }

    pub fn my_other_units(&self) -> Vec<&Unit> {
        get_game()
            .my_units()
            .into_iter()
            .filter(|e| self.id != e.id)
            .collect_vec()
    }

    pub fn my_closest_other_unit(&self) -> Option<(f64, &Unit)> {
        get_game()
            .my_units()
            .into_iter()
            .filter(|e| self.id != e.id)
            .filter(|e| e.remaining_spawn_time.is_none())
            .map(|e| (e.position.distance(&self.position), e))
            .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap())
    }

    pub fn is_inside_vision(&self, p: &Vec2) -> bool {
        if self.position.distance(p) > get_constants().view_distance {
            return false;
        };
        let (left_angle, right_angle) = self.view_segment_angles();
        let angle = (p.clone() - self.position.clone()).angle();
        (left_angle >= angle && right_angle <= angle)
            || (left_angle <= angle && right_angle >= angle)
    }

    pub fn firing_distance(&self) -> f64 {
        match self.weapon {
            None => 0.0,
            Some(w) => get_constants().weapons[w as usize].firing_distance(),
        }
    }
    pub fn points_in_radius(&self, radius: i32) -> Vec<Vec2> {
        let mut res = Vec::new();
        for x in 0..(radius * 2 + 1) {
            for y in 0..(radius * 2 + 1) {
                let p = Vec2 {
                    x: self.position.x + x as f64 - radius as f64,
                    y: self.position.y + y as f64 - radius as f64,
                };
                if self.position.distance(&p) > radius as f64 {
                    continue;
                }
                res.push(p);
            }
        }
        return res;
    }
    pub fn points_around_unit(&self, check_obstacles: bool) -> Vec<Vec2> {
        let points_around = 10;
        let constants = get_constants();

        let forward_point = self.position.clone()
            + (self.direction.clone()
            * (constants.max_unit_forward_speed / constants.ticks_per_second * 2.0));
        let backward_point = self.position.clone()
            - (self.direction.clone()
            * (constants.max_unit_backward_speed / constants.ticks_per_second * 2.0));
        let center = (forward_point.clone() + backward_point) / 2.0;
        let radius = forward_point.distance(&center);

        let angle_diff = 2.0 * PI / points_around as f64;
        let mut res = Vec::new();
        let mut cur_angle = self.direction.angle();
        let obstacles = get_obstacles(self.id);
        for _ in 0..points_around {
            let next_vec = rotate(center.clone(), cur_angle, radius);
            let next_vec_after_some_ticks = rotate(center.clone(), cur_angle, radius * 5.0);
            let intersects_with_obstacles = obstacles
                .iter()
                .find(|o| {
                    o.position.distance(&next_vec_after_some_ticks)
                        < o.radius + get_constants().unit_radius
                })
                .is_some();
            if !intersects_with_obstacles || !check_obstacles {
                res.push(next_vec);
            }
            cur_angle += angle_diff;
        }

        res
    }

    pub fn view_segment_angles(&self) -> (f64, f64) {
        let default_view = get_constants().field_of_view;
        let view_angle = self
            .weapon
            .map(|e| {
                default_view
                    - (default_view - get_constants().weapons[e as usize].aim_field_of_view)
                    * self.aim
            })
            .unwrap_or(default_view)
            * PI
            / 180.0;

        let left_angle = self.direction.angle() - view_angle / 2.0;
        let right_angle = self.direction.angle() + view_angle / 2.0;
        (left_angle, right_angle)
    }

    pub fn view_segment(&self) -> (Vec2, Vec2) {
        let (left_angle, right_angle) = self.view_segment_angles();

        let first = rotate(
            self.position.clone(),
            left_angle,
            get_constants().view_distance,
        );
        let second = rotate(
            self.position.clone(),
            right_angle,
            get_constants().view_distance,
        );
        (first, second)
    }
}

impl trans::Trans for Unit {
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.id.write_to(writer)?;
        self.player_id.write_to(writer)?;
        self.health.write_to(writer)?;
        self.shield.write_to(writer)?;
        self.extra_lives.write_to(writer)?;
        self.position.write_to(writer)?;
        self.remaining_spawn_time.write_to(writer)?;
        self.velocity.write_to(writer)?;
        self.direction.write_to(writer)?;
        self.aim.write_to(writer)?;
        self.action.write_to(writer)?;
        self.health_regeneration_start_tick.write_to(writer)?;
        self.weapon.write_to(writer)?;
        self.next_shot_tick.write_to(writer)?;
        self.ammo.write_to(writer)?;
        self.shield_potions.write_to(writer)?;
        Ok(())
    }
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let id: i32 = trans::Trans::read_from(reader)?;
        let player_id: i32 = trans::Trans::read_from(reader)?;
        let health: f64 = trans::Trans::read_from(reader)?;
        let shield: f64 = trans::Trans::read_from(reader)?;
        let extra_lives: i32 = trans::Trans::read_from(reader)?;
        let position: model::Vec2 = trans::Trans::read_from(reader)?;
        let remaining_spawn_time: Option<f64> = trans::Trans::read_from(reader)?;
        let velocity: model::Vec2 = trans::Trans::read_from(reader)?;
        let direction: model::Vec2 = trans::Trans::read_from(reader)?;
        let aim: f64 = trans::Trans::read_from(reader)?;
        let action: Option<model::Action> = trans::Trans::read_from(reader)?;
        let health_regeneration_start_tick: i32 = trans::Trans::read_from(reader)?;
        let weapon: Option<i32> = trans::Trans::read_from(reader)?;
        let next_shot_tick: i32 = trans::Trans::read_from(reader)?;
        let ammo: Vec<i32> = trans::Trans::read_from(reader)?;
        let shield_potions: i32 = trans::Trans::read_from(reader)?;
        Ok(Self {
            id,
            player_id,
            health,
            shield,
            extra_lives,
            position,
            remaining_spawn_time,
            velocity,
            direction,
            aim,
            action,
            health_regeneration_start_tick,
            weapon,
            next_shot_tick,
            ammo,
            shield_potions,
        })
    }
}
