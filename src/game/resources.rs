use crate::constants::{NIGHT_DURATION, RESOURCE_FACTOR, SOLDIER_BASE_DAMAGE};
use crate::game::enemy::components::Size;
use crate::game::weapon::components::{AirFireStrategy, FireStrategy, MortarShell, WeaponName};
use bevy::ecs::system::SystemId;
use bevy::prelude::{default, Resource, Timer};
use bevy::time::TimerMode;
use bevy::utils::hashbrown::HashMap;
use rand::random;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use uuid::Uuid;

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
    pub system: Option<SystemId>,
    pub day_tab: DayTabs,
    pub enemy_info: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            speed: 1.,
            system: None,
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
}

pub struct Fence {
    pub health: f32,
    pub max_health: f32,
    pub enabled: bool,
    pub damage: f32,
    pub cost: Resources,
}

pub struct Spotlight {
    pub power: u32,
    pub cost: Resources,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

pub struct Constructions {
    pub armory: u32,
    pub refinery: u32,
    pub factory: u32,
    pub laboratory: u32,
}

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

#[derive(Hash)]
pub struct Spot {
    pub id: Uuid,
    pub weapon: Option<WeaponName>,
}

pub struct Weapons {
    pub spots: Vec<Spot>,
    pub mines: u32,
    pub bombs: u32,
    pub nuke: u32,
    pub settings: WeaponSettings,
}

#[derive(Clone, Copy, Debug, EnumIter, Hash, Eq, PartialEq)]
pub enum TechnologyName {
    Spotlight,
    Electricity,
    Physics,
    Marines,
    Aimbot,
    Explosives,
    Homing,
    Charts,
    Productivity,
}

#[derive(Clone, Copy, Debug, EnumIter, Hash, Eq, PartialEq)]
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

impl Technology {
    pub fn get(name: TechnologyName) -> Self {
        match name {
            TechnologyName::Spotlight => Self {
                name,
                price: 100.,
                category: TechnologyCategory::Science,
                description: "\
                    Enables the spotlight during the night. The spotlight \
                    increases the vision of the player, allowing weapons to \
                    shoot earlier. Using it costs gasoline.",
            },
            TechnologyName::Electricity => Self {
                name: TechnologyName::Electricity,
                price: 500.,
                category: TechnologyCategory::Science,
                description: "\
                        Enables the option to electrify the fence, doing damage to adjacent enemies.",
            },
            TechnologyName::Physics => Self {
                name,
                price: 1000.,
                category: TechnologyCategory::Science,
                description: "Unlocks the nuke.",
            },
            TechnologyName::Marines => Self {
                name,
                price: 100.,
                category: TechnologyCategory::Military,
                description: "Doubles the strength of your soldiers.",
            },
            TechnologyName::Aimbot => Self {
                name,
                price: 200.,
                category: TechnologyCategory::Military,
                description: "\
                    Predict the movement of enemies, shooting at the position where \
                    an enemy is going to be when the bullet arrives. Not relevant for \
                    homing bullets.",
            },
            TechnologyName::Explosives => Self {
                name: TechnologyName::Explosives,
                price: 400.,
                category: TechnologyCategory::Military,
                description: "Unlocks mines and bombs.",
            },
            TechnologyName::Homing => Self {
                name: TechnologyName::Homing,
                price: 500.,
                category: TechnologyCategory::Military,
                description: "\
                Homing bullets are directed to a specific enemy and follow its movement.\
                Unlocks homing weapons turret and missile launcher.",
            },
            TechnologyName::Charts => Self {
                name: TechnologyName::Charts,
                price: 100.,
                category: TechnologyCategory::Economy,
                description: "\
                        Enables sending expeditions. Expeditions cost gasoline, materials and \
                        population, but can yield interesting rewards after some days.",
            },
            TechnologyName::Productivity => Self {
                name,
                price: 1000.,
                category: TechnologyCategory::Economy,
                description: "Armorers, refiners and constructors produce 50% more resources.",
            },
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        TechnologyName::iter().map(Self::get)
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Hash, Eq, PartialEq)]
pub enum ExpeditionName {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, Debug)]
pub struct ExpeditionReward {
    pub population: u32,
    pub resources: Resources,
    pub mines: u32,
    pub bombs: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum ExpeditionStatus {
    /// The expedition is still ongoing
    Ongoing,

    /// The expedition did not return
    Lost,

    /// The expedition returns with reward
    Returned(ExpeditionReward),
}

#[derive(Clone, Copy)]
pub struct Expedition {
    pub name: ExpeditionName,
    pub duration: &'static str,
    pub day: u32,
    pub max_day: u32,
    pub return_prob: f32,
    pub population: u32,
    pub price: Resources,
    pub status: ExpeditionStatus,
}

impl Expedition {
    pub fn get(name: ExpeditionName) -> Self {
        match name {
            ExpeditionName::Small => Self {
                name,
                duration: "1-2 days",
                day: 0,
                max_day: 3,
                return_prob: 0.7,
                population: 25,
                price: Resources {
                    gasoline: 150.,
                    materials: 75.,
                    ..default()
                },
                status: ExpeditionStatus::Ongoing,
            },
            ExpeditionName::Medium => Self {
                name,
                duration: "1-3 days",
                day: 0,
                max_day: 4,
                return_prob: 0.5,
                population: 75,
                price: Resources {
                    gasoline: 300.,
                    materials: 150.,
                    ..default()
                },
                status: ExpeditionStatus::Ongoing,
            },
            ExpeditionName::Large => Self {
                name,
                duration: "2-4 days",
                day: 0,
                max_day: 5,
                return_prob: 0.3,
                population: 125,
                price: Resources {
                    gasoline: 450.,
                    materials: 225.,
                    ..default()
                },
                status: ExpeditionStatus::Ongoing,
            },
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        ExpeditionName::iter().map(Self::get)
    }

    pub fn update(&mut self) {
        self.day += 1;

        if self.day == self.max_day {
            self.status = ExpeditionStatus::Lost;
        } else if random::<f32>() < self.return_prob {
            self.status = ExpeditionStatus::Returned(ExpeditionReward {
                population: ((self.population * self.day.pow(3)) as f32 * random::<f32>() + 0.5)
                    as u32,
                resources: Resources {
                    bullets: (self.population * self.day.pow(3)) as f32 * random::<f32>() + 0.5,
                    gasoline: (self.population * self.day.pow(3).pow(2)) as f32 * random::<f32>()
                        + 0.5,
                    materials: (self.population * self.day.pow(3).pow(2)) as f32 * random::<f32>()
                        + 0.5,
                    technology: (self.population * self.day.pow(3).pow(2)) as f32 * random::<f32>()
                        + 0.5,
                },
                mines: (self.day.pow(2) as f32 * random::<f32>()) as u32,
                bombs: (self.day as f32 * random::<f32>()) as u32,
            });
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
    pub constructions: Constructions,
    pub weapons: Weapons,
    pub technology: HashSet<TechnologyName>,
    pub expedition: Option<Expedition>,
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
            },
            fence: Fence {
                health: 0.,
                max_health: 0.,
                enabled: false,
                damage: 5.,
                cost: Resources {
                    gasoline: 2.,
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
                bullets: 10_000.,
                gasoline: 10_000.,
                materials: 10_000.,
                technology: 10_000.,
            },
            constructions: Constructions {
                armory: 1,
                refinery: 1,
                factory: 1,
                laboratory: 1,
            },
            weapons: Weapons {
                spots: vec![
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::Mortar),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::Turret),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::Flamethrower),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::Canon),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::AAA),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::Artillery),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::MachineGun),
                    },
                    Spot {
                        id: Uuid::new_v4(),
                        weapon: Some(WeaponName::MissileLauncher),
                    },
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
            expedition: None,
            stats: HashMap::default(),
        }
    }

    pub fn new_resources(&self) -> Resources {
        let productivity = if self.has_tech(TechnologyName::Productivity) {
            1.5
        } else {
            1.
        };

        Resources {
            bullets: (self.population.armorer * self.constructions.armory * RESOURCE_FACTOR) as f32
                * productivity,
            gasoline: (self.population.refiner * self.constructions.refinery * RESOURCE_FACTOR)
                as f32
                * productivity,
            materials: (self.population.constructor * self.constructions.factory * RESOURCE_FACTOR)
                as f32
                * productivity,
            technology: (self.population.scientist
                * self.constructions.laboratory
                * RESOURCE_FACTOR) as f32
                * productivity,
        }
    }

    pub fn has_tech(&self, tech: TechnologyName) -> bool {
        self.technology.contains(&tech)
    }

    pub fn get_soldier_damage(&self) -> u32 {
        if self.has_tech(TechnologyName::Marines) {
            2 * SOLDIER_BASE_DAMAGE
        } else {
            SOLDIER_BASE_DAMAGE
        }
    }

    pub fn resolve_expedition(&mut self) {
        if let Some(ref mut expedition) = self.expedition {
            match expedition.status {
                ExpeditionStatus::Returned(reward) => {
                    self.population.idle += reward.population;
                    self.resources += &reward.resources;
                    self.weapons.mines += reward.mines;
                    self.weapons.bombs += reward.bombs;

                    self.expedition = None;
                }
                ExpeditionStatus::Lost => self.expedition = None,
                ExpeditionStatus::Ongoing => (),
            }
        }
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
