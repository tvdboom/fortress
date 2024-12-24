use crate::game::resources::WaveStats;
use bevy::prelude::Resource;
use crate::game::weapon::components::*;

#[derive(Clone)]
pub struct Resources {
    pub bullets: u32,
    pub gasoline: u32,
    pub materials: u32,
}

impl Default for Resources {
    fn default() -> Self {
        Self {bullets: 0, gasoline: 0, materials: 0}
    }
}

pub struct Weapons {
    pub n_sentry_gun: u32,
    pub sentry_gun: SentryGun,
}

impl Default for Weapons {
    fn default() -> Self {
        Self {
            n_sentry_gun: 1,
            sentry_gun: SentryGun::default(),
        }
    }
}

pub struct Wall {
    pub max_health: u32,
    pub health: u32,
}

impl Default for Wall {
    fn default() -> Self {
        Self {max_health: 1000, health: 1000}
    }
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
            resources: Resources {bullets: 1000, gasoline: 1000, materials: 1000},
            weapons: Weapons {
                n_sentry_gun: 1,
                sentry_gun: SentryGun::default(),
            },
            wall: Wall::default(),
            stats: vec![],
        }
    }
}