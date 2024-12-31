use crate::game::enemy::components::EnemyType;
use crate::game::weapon::components::WeaponId;
use bevy::prelude::{Resource, Timer};
use bevy::time::TimerMode;
use bevy::utils::hashbrown::HashMap;

#[derive(Resource)]
pub struct GameSettings {
    pub speed: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self { speed: 1. }
    }
}

#[derive(Clone)]
pub struct Resources {
    pub bullets: u32,
    pub gasoline: u32,
    pub materials: u32,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            bullets: 0,
            gasoline: 0,
            materials: 0,
        }
    }
}

pub struct Wall {
    pub max_health: f32,
    pub health: f32,
    pub max_spots: u32,
}

#[derive(Resource)]
pub struct Player {
    pub day: u32,
    pub resources: Resources,
    pub wall: Wall,
    pub weapons: Vec<Option<WeaponId>>,
    pub stats: HashMap<u32, WaveStats>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            day: 1,
            resources: Resources {
                bullets: 1_000,
                gasoline: 1_000,
                materials: 1_000,
            },
            wall: Wall {
                max_health: 1_000.,
                health: 1_000.,
                max_spots: 5,
            },
            weapons: vec![
                None,
                Some(WeaponId::SentryGun),
                None,
                Some(WeaponId::SentryGun),
                None,
            ],
            stats: HashMap::default(),
        }
    }
}

#[derive(Clone)]
pub struct EnemyStatus {
    pub spawned: u32,
    pub killed: u32,
}

#[derive(Resource, Clone)]
pub struct WaveStats {
    pub day: u32,
    pub time: Timer,
    pub resources: Resources,
    pub enemies: HashMap<EnemyType, EnemyStatus>,
}

impl Default for WaveStats {
    fn default() -> Self {
        Self {
            day: 1,
            time: Timer::from_seconds(60., TimerMode::Once),
            resources: Resources::default(),
            enemies: HashMap::default(),
        }
    }
}
