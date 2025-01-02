use crate::constants::NIGHT_DURATION;
use crate::game::enemy::components::EnemyType;
use crate::game::weapon::components::{Weapon, WeaponSettings};
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
    pub bullets: f32,
    pub gasoline: f32,
    pub materials: f32,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            bullets: 0.,
            gasoline: 0.,
            materials: 0.,
        }
    }
}

pub struct Wall {
    pub health: f32,
    pub max_health: f32,
    pub upgrade_price: Resources, // Upgrade => +10k max health
    pub repair_price: Resources,  // Repair => +1k health
}

pub struct Fence {
    pub health: f32,
    pub max_health: f32,
    pub enabled: bool,
    pub damage: f32,
    pub cost: Resources,
    pub upgrade_price: Resources, // Upgrade => +100 max health
    pub repair_price: Resources,  // Repair => +20 health
}

pub struct Weapons {
    pub settings: WeaponSettings,
    pub spots: Vec<Option<Weapon>>,
}

#[derive(Resource)]
pub struct Player {
    pub day: u32,
    pub survivors: u32,
    pub wall: Wall,
    pub fence: Fence,
    pub resources: Resources,
    pub weapons: Weapons,
    pub stats: HashMap<u32, NightStats>,
}

impl Player {
    pub fn init(game_settings: &GameSettings) -> Self {
        let weapon_settings = WeaponSettings::default();
        Self {
            day: 1,
            survivors: 100,
            wall: Wall {
                health: 1_000.,
                max_health: 1_000.,
                upgrade_price: Resources {
                    materials: 10.,
                    ..Resources::default()
                },
                repair_price: Resources {
                    materials: 1.,
                    ..Resources::default()
                },
            },
            fence: Fence {
                health: 300.,
                max_health: 300.,
                enabled: false,
                damage: 5.,
                cost: Resources {
                    gasoline: 2.,
                    ..Resources::default()
                },
                upgrade_price: Resources {
                    materials: 1.,
                    ..Resources::default()
                },
                repair_price: Resources {
                    materials: 1.,
                    ..Resources::default()
                },
            },
            resources: Resources {
                bullets: 1_000.,
                gasoline: 1_000.,
                materials: 1_000.,
            },
            weapons: Weapons {
                settings: weapon_settings.clone(),
                spots: vec![
                    None,
                    Some(Weapon::sentry_gun(&weapon_settings, game_settings)),
                    None,
                    Some(Weapon::sentry_gun(&weapon_settings, game_settings)),
                    None,
                ],
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
pub struct NightStats {
    pub day: u32,
    pub timer: Timer,
    pub resources: Resources,
    pub enemies: HashMap<EnemyType, EnemyStatus>,
}

impl Default for NightStats {
    fn default() -> Self {
        Self {
            day: 1,
            timer: Timer::from_seconds(NIGHT_DURATION, TimerMode::Once),
            resources: Resources::default(),
            enemies: HashMap::default(),
        }
    }
}
