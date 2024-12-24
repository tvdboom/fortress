use crate::game::resources::WaveStats;
use crate::game::weapon::components::*;
use bevy::prelude::Resource;

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
    pub resources: Resources,
    pub weapons: Weapons,
    pub wall: Wall,
    pub stats: Vec<WaveStats>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            resources: Resources {
                bullets: 1000,
                gasoline: 1000,
                materials: 1000,
            },
            weapons: Weapons { sentry_gun: 2 },
            wall: Wall {
                max_health: 10_000,
                health: 10_000,
            },
            stats: vec![],
        }
    }
}
