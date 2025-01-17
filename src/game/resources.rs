use crate::constants::NIGHT_DURATION;
use crate::game::enemy::components::Size;
use crate::game::weapon::components::{AirFireStrategy, FireStrategy, MortarShell, WeaponName};
use bevy::prelude::{Resource, Timer};
use bevy::time::TimerMode;
use bevy::utils::hashbrown::HashMap;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq)]
pub enum DayTabs {
    Overview,
    Population,
    Constructions,
    Armory,
    Technology,
    Expeditions,
}

#[derive(Resource)]
pub struct GameSettings {
    pub speed: f32,
    pub day_tab: DayTabs,
    pub enemy_info: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            speed: 1.,
            day_tab: DayTabs::Overview,
            enemy_info: false,
        }
    }
}

#[derive(Clone)]
pub struct Population {
    pub soldier: u32,
    pub armorer: u32,
    pub refiner: u32,
    pub constructor: u32,
    pub scientist: u32,
    pub idle: u32,
}

impl Population {
    pub fn total(&self) -> u32 {
        self.soldier + self.armorer + self.refiner + self.constructor + self.scientist + self.idle
    }
}

impl Default for Population {
    fn default() -> Self {
        Self {
            soldier: 0,
            armorer: 0,
            refiner: 0,
            constructor: 0,
            scientist: 0,
            idle: 0,
        }
    }
}

pub struct Wall {
    pub health: f32,
    pub max_health: f32,
    pub upgrade_price: Resources,
    pub repair_price: Resources,
}

pub struct Fence {
    pub health: f32,
    pub max_health: f32,
    pub enabled: bool,
    pub damage: f32,
    pub cost: Resources,
    pub upgrade_price: Resources,
    pub repair_price: Resources,
}

pub struct Spotlight {
    pub power: u32,
    pub cost: Resources,
}

#[derive(Clone, PartialEq)]
pub struct Resources {
    pub bullets: f32,
    pub gasoline: f32,
    pub materials: f32,
    pub technology: f32,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            bullets: 0.,
            gasoline: 0.,
            materials: 0.,
            technology: 0.,
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
            // Binary operations with Resources reference
            impl $trait<&Self> for Resources {
                type Output = Self;

                fn $method(self, rhs: &Resources) -> Self::Output {
                    Self {
                        bullets: self.bullets $op rhs.bullets,
                        gasoline: self.gasoline $op rhs.gasoline,
                        materials: self.materials $op rhs.materials,
                        technology: self.technology $op rhs.technology,
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
                        technology: self.technology $op float,
                    }
                }
            }

            // Binary operations with float on reference
            impl<T: Into<f32>> $trait<T> for &Resources {
                type Output = Resources;

                fn $method(self, rhs: T) -> Resources {
                    let float = rhs.into();
                    Resources {
                        bullets: self.bullets $op float,
                        gasoline: self.gasoline $op float,
                        materials: self.materials $op float,
                        technology: self.technology $op float,
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
            // Assignment operations with Resources reference
            impl $trait<&Self> for Resources {
                fn $method(&mut self, rhs: &Self) {
                    self.bullets $op rhs.bullets;
                    self.gasoline $op rhs.gasoline;
                    self.materials $op rhs.materials;
                    self.technology $op rhs.technology;
                }
            }

            // Assignment operations with float
            impl<T: Into<f32>> $trait<T> for Resources {
                fn $method(&mut self, rhs: T) {
                    let float = rhs.into();
                    self.bullets $op float;
                    self.gasoline $op float;
                    self.materials $op float;
                    self.technology $op float;
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

#[derive(Clone)]
pub struct WeaponSettings {
    pub aaa: AirFireStrategy,
    pub artillery: FireStrategy,
    pub canon: AirFireStrategy,
    pub flamethrower: u32,
    pub machine_gun: u32,
    pub missile_launcher: u32,
    pub mortar: MortarShell,
    pub turret: f32,
    pub bomb: FireStrategy,
    pub mine: Size,
}

pub struct Weapons {
    pub spots: Vec<Option<WeaponName>>,
    pub mines: u32,
    pub bombs: u32,
    pub nuke: u32,
    pub settings: WeaponSettings,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TechnologyName {
    Spotlight,
    AimBot,
}

#[derive(EnumIter, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TechnologyCategory {
    Science,
    Military,
    Economy,
}

pub struct Technology {
    pub name: TechnologyName,
    pub price: f32,
    pub category: TechnologyCategory,
    pub description: &'static str,
}

#[derive(Resource)]
pub struct TechnologyManager {
    pub list: Vec<Technology>,
}

impl Default for TechnologyManager {
    fn default() -> Self {
        Self {
            list: vec![
                Technology {
                    name: TechnologyName::Spotlight,
                    price: 100.,
                    category: TechnologyCategory::Science,
                    description: "\
                        Enables the spotlight during the night. The spotlight \
                        increases the vision of the player, allowing weapons to \
                        shoot earlier. Using it costs gasoline.",
                },
                Technology {
                    name: TechnologyName::AimBot,
                    price: 200.,
                    category: TechnologyCategory::Military,
                    description: "\
                        Predict the movement of enemies, shooting at the position where \
                        an enemy is going to be when the bullet arrives. Not relevant for \
                        homing bullets.",
                },
            ],
        }
    }
}

#[derive(Resource)]
pub struct Player {
    pub day: u32,
    pub population: Population,
    pub wall: Wall,
    pub fence: Fence,
    pub spotlight: Spotlight,
    pub resources: Resources,
    pub weapons: Weapons,
    pub technology: HashSet<TechnologyName>,
    pub stats: HashMap<u32, NightStats>,
}

impl Player {
    pub fn init() -> Self {
        Self {
            day: 1,
            population: Population {
                soldier: 5,
                armorer: 20,
                refiner: 20,
                constructor: 20,
                scientist: 5,
                idle: 0,
            },
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
                technology: 0.,
            },
            weapons: Weapons {
                spots: vec![
                    Some(WeaponName::Canon),
                    Some(WeaponName::Mortar),
                    Some(WeaponName::AAA),
                    Some(WeaponName::MissileLauncher),
                    Some(WeaponName::Flamethrower),
                    Some(WeaponName::Turret),
                    Some(WeaponName::MachineGun),
                    Some(WeaponName::Artillery),
                ],
                mines: 10,
                bombs: 10,
                nuke: 5,
                settings: WeaponSettings {
                    aaa: AirFireStrategy::None,
                    artillery: FireStrategy::None,
                    canon: AirFireStrategy::None,
                    flamethrower: 0,
                    machine_gun: 0,
                    mortar: MortarShell::None,
                    turret: 0.,
                    missile_launcher: 0,
                    bomb: FireStrategy::Density,
                    mine: Size::Medium,
                },
            },
            technology: HashSet::default(),
            stats: HashMap::default(),
        }
    }

    pub fn has_tech(&self, tech: TechnologyName) -> bool {
        self.technology.contains(&tech)
    }
}

#[derive(Clone)]
pub struct EnemyStatus {
    pub spawned: u32,
    pub killed: u32,
}

#[derive(Clone)]
pub struct ResourcesWarnings {
    pub low_bullets: bool,
    pub no_bullets: bool,
    pub low_gasoline: bool,
    pub no_gasoline: bool,
}

#[derive(Resource, Clone)]
pub struct NightStats {
    pub day: u32,
    pub timer: Timer,
    pub spawn_timer: Timer,
    pub population: Population,
    pub resources: Resources,
    pub enemies: HashMap<&'static str, EnemyStatus>,
    pub warnings: ResourcesWarnings,
}

impl Default for NightStats {
    fn default() -> Self {
        Self {
            day: 1,
            timer: Timer::from_seconds(NIGHT_DURATION, TimerMode::Once),
            spawn_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            population: Population::default(),
            resources: Resources::default(),
            enemies: HashMap::default(),
            warnings: ResourcesWarnings {
                low_bullets: false,
                no_bullets: false,
                low_gasoline: false,
                no_gasoline: false,
            },
        }
    }
}
