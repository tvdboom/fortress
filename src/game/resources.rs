use crate::game::enemy::components::EnemyType;
use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;

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

pub struct SentryGunSettings {
    pub amount: u32,
    pub fire_rate: u32,
    pub fire_rate_min: u32,
    pub fire_rate_max: u32,
}

pub struct WeaponSettings {
    pub sentry_gun: SentryGunSettings,
}

pub struct Wall {
    pub max_health: u32,
    pub health: u32,
    pub max_spots: u32,
}

#[derive(Resource)]
pub struct Player {
    pub day: u32,
    pub speed: f32,
    pub resources: Resources,
    pub wall: Wall,
    pub weapons: WeaponSettings,
    pub stats: HashMap<u32, WaveStats>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            day: 1,
            speed: 1.,
            resources: Resources {
                bullets: 1000,
                gasoline: 1000,
                materials: 1000,
            },
            wall: Wall {
                max_health: 1_000,
                health: 1_000,
                max_spots: 5,
            },
            weapons: WeaponSettings {
                sentry_gun: SentryGunSettings {
                    amount: 2,
                    fire_rate: 1,
                    fire_rate_min: 0,
                    fire_rate_max: 5,
                },
            },
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
    pub resources: Resources,
    pub enemies: HashMap<EnemyType, EnemyStatus>,
}

impl Default for WaveStats {
    fn default() -> Self {
        Self {
            day: 1,
            resources: Resources::default(),
            enemies: HashMap::default(),
        }
    }
}
