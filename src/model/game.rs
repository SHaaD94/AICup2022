use super::*;
use crate::strategy::holder::get_constants;
use itertools::Itertools;

/// Current game's state
#[derive(Clone, Debug)]
pub struct Game {
    /// Your player's id
    pub my_id: i32,
    /// List of players (teams)
    pub players: Vec<model::Player>,
    /// Current tick
    pub current_tick: i32,
    /// List of units visible by your team
    pub units: Vec<model::Unit>,
    /// List of loot visible by your team
    pub loot: Vec<model::Loot>,
    /// List of projectiles visible by your team
    pub projectiles: Vec<model::Projectile>,
    /// Current state of game zone
    pub zone: model::Zone,
    /// List of sounds heard by your team during last tick
    pub sounds: Vec<model::Sound>,
}

impl Game {
    pub const fn const_default() -> Self {
        Game {
            my_id: 0,
            players: vec![],
            current_tick: 0,
            units: vec![],
            loot: vec![],
            projectiles: vec![],
            zone: Zone {
                current_center: Vec2 { x: 0.0, y: 0.0 },
                current_radius: 0.0,
                next_center: Vec2 { x: 1.0, y: 0.0 },
                next_radius: 0.0,
            },
            sounds: vec![],
        }
    }

    pub fn my_units(&self) -> Vec<&Unit> {
        self.units
            .iter()
            .filter(|e| e.player_id == self.my_id)
            .collect_vec()
    }
    pub fn enemy_units(&self) -> Vec<&Unit> {
        self.units
            .iter()
            .filter(|e| e.player_id != self.my_id)
            .collect_vec()
    }
    pub fn intersecting_loot(&self, unit: &Unit) -> Vec<&Loot> {
        self.loot
            .iter()
            .filter(|l| unit.position.distance(&l.position.clone()) < get_constants().unit_radius)
            .collect_vec()
    }
}

impl trans::Trans for Game {
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.my_id.write_to(writer)?;
        self.players.write_to(writer)?;
        self.current_tick.write_to(writer)?;
        self.units.write_to(writer)?;
        self.loot.write_to(writer)?;
        self.projectiles.write_to(writer)?;
        self.zone.write_to(writer)?;
        self.sounds.write_to(writer)?;
        Ok(())
    }
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let my_id: i32 = trans::Trans::read_from(reader)?;
        let players: Vec<model::Player> = trans::Trans::read_from(reader)?;
        let current_tick: i32 = trans::Trans::read_from(reader)?;
        let units: Vec<model::Unit> = trans::Trans::read_from(reader)?;
        let loot: Vec<model::Loot> = trans::Trans::read_from(reader)?;
        let projectiles: Vec<model::Projectile> = trans::Trans::read_from(reader)?;
        let zone: model::Zone = trans::Trans::read_from(reader)?;
        let sounds: Vec<model::Sound> = trans::Trans::read_from(reader)?;
        Ok(Self {
            my_id,
            players,
            current_tick,
            units,
            loot,
            projectiles,
            zone,
            sounds,
        })
    }
}
