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

pub struct Weapons {
    pub sentry_gun: u32,
}

pub struct Wall {
    pub max_health: u32,
    pub health: u32,
}

#[derive(Resource)]
pub struct Player {
    pub day: u32,
    pub resources: Resources,
    pub weapons: Weapons,
    pub wall: Wall,
    pub stats: HashMap<u32, WaveStats>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            day: 1,
            resources: Resources {
                bullets: 1000,
                gasoline: 1000,
                materials: 1000,
            },
            weapons: Weapons { sentry_gun: 2 },
            wall: Wall {
                max_health: 1_000,
                health: 1_000,
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
    pub enemies: HashMap<String, EnemyStatus>,
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
