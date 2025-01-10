use crate::constants::NIGHT_DURATION;
use crate::game::enemy::components::Size;
use crate::game::weapon::components::{AAAFireStrategy, FireStrategy, MortarShell, WeaponName};
use bevy::prelude::{Resource, Timer};
use bevy::time::TimerMode;
use bevy::utils::hashbrown::HashMap;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Resource)]
pub struct GameSettings {
    pub speed: f32,
    pub enemy_info: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            speed: 1.,
            enemy_info: false,
        }
    }
}

#[derive(Clone, PartialEq)]
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

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let bullets_cmp = self.bullets.partial_cmp(&other.bullets)?;
        let gasoline_cmp = self.gasoline.partial_cmp(&other.gasoline)?;
        let materials_cmp = self.materials.partial_cmp(&other.materials)?;

        if bullets_cmp == gasoline_cmp && gasoline_cmp == materials_cmp {
            Some(bullets_cmp)
        } else {
            None
        }
    }
}

macro_rules! resources_binary_ops {
    ($($trait:ident, $method:ident, $op:tt);*;) => {
        $(
            // Binary operations with Resources
            impl $trait for Resources {
                type Output = Self;

                fn $method(self, rhs: Self) -> Self::Output {
                    Self {
                        bullets: self.bullets $op rhs.bullets,
                        gasoline: self.gasoline $op rhs.gasoline,
                        materials: self.materials $op rhs.materials,
                    }
                }
            }

            // Binary operations with float
            impl<T: Into<f32>> $trait<T> for Resources {
                type Output = Self;

                fn $method(self, rhs: T) -> Self::Output {
                    let float = rhs.into();
                    Self {
                        bullets: self.bullets $op float,
                        gasoline: self.gasoline $op float,
                        materials: self.materials $op float,
                    }
                }
            }
        )*
    };
}

resources_binary_ops!(
    Add, add, +;
    Sub, sub, -;
    Mul, mul, *;
    Div, div, /;
);


macro_rules! resources_assignment_ops {
    ($($trait:ident, $method:ident, $op:tt);*;) => {
        $(
            // Assignment operations with Resources
            impl $trait<&Self> for Resources {
                fn $method(&mut self, rhs: &Self) {
                    self.bullets $op rhs.bullets;
                    self.gasoline $op rhs.gasoline;
                    self.materials $op rhs.materials;
                }
            }

            // Assignment operations with float
            impl<T: Into<f32>> $trait<T> for Resources {
                fn $method(&mut self, rhs: T) {
                    let float = rhs.into();
                    self.bullets $op float;
                    self.gasoline $op float;
                    self.materials $op float;
                }
            }
        )*
    };
}

resources_assignment_ops!(
    AddAssign, add_assign, +=;
    SubAssign, sub_assign, -=;
    MulAssign, mul_assign, *=;
    DivAssign, div_assign, /=;
);


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

pub struct Spotlight {
    pub power: u32,
    pub cost: Resources,
}

pub struct WeaponSettings {
    pub sentry_gun_fire_rate: u32,
    pub flamethrower_power: u32,
    pub aaa_fire_strategy: AAAFireStrategy,
    pub mortar_shell: MortarShell,
    pub turret_fire_strategy: FireStrategy,
    pub mine_sensibility: Size,
}

pub struct Weapons {
    pub spots: Vec<Option<WeaponName>>,
    pub mines: u32,
    pub settings: WeaponSettings,
}

pub struct Technology {
    pub spotlight: bool,
    pub movement_prediction: bool,
}

#[derive(Resource)]
pub struct Player {
    pub day: u32,
    pub survivors: u32,
    pub wall: Wall,
    pub fence: Fence,
    pub spotlight: Spotlight,
    pub resources: Resources,
    pub weapons: Weapons,
    pub technology: Technology,
    pub stats: HashMap<u32, NightStats>,
}

impl Player {
    pub fn init() -> Self {
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
            spotlight: Spotlight {
                power: 0,
                cost: Resources {
                    gasoline: 0.1, // Cost per second per power
                    ..Resources::default()
                },
            },
            resources: Resources {
                bullets: 1_000.,
                gasoline: 1_000.,
                materials: 1_000.,
            },
            weapons: Weapons {
                spots: vec![
                    Some(WeaponName::MachineGun),
                    Some(WeaponName::Mortar),
                    Some(WeaponName::AAA),
                    Some(WeaponName::Flamethrower),
                    Some(WeaponName::Turret),
                    Some(WeaponName::MachineGun),
                ],
                mines: 0,
                settings: WeaponSettings {
                    sentry_gun_fire_rate: 0,
                    flamethrower_power: 0,
                    aaa_fire_strategy: AAAFireStrategy::None,
                    mortar_shell: MortarShell::None,
                    turret_fire_strategy: FireStrategy::None,
                    mine_sensibility: Size::Medium,
                },
            },
            technology: Technology {
                spotlight: true,
                movement_prediction: true,
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
    pub spawn_timer: Timer,
    pub resources: Resources,
    pub enemies: HashMap<&'static str, EnemyStatus>,
}

impl Default for NightStats {
    fn default() -> Self {
        Self {
            day: 1,
            timer: Timer::from_seconds(NIGHT_DURATION, TimerMode::Once),
            spawn_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            resources: Resources::default(),
            enemies: HashMap::default(),
        }
    }
}
