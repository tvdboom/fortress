use crate::constants::{BETA, NIGHT_DURATION, NO_SPAWN_START, NO_SPAWN_STEP};
use bevy::prelude::*;
use bevy::prelude::{Resource, Vec2};
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Size {
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Component, Copy, Clone)]
pub struct Enemy {
    pub name: &'static str,
    pub image: &'static str,
    pub max_health: f32,
    pub health: f32,
    pub dim: Vec2,
    pub size: Size,
    pub armor: f32,
    pub speed: f32,
    pub can_fly: bool,
    pub damage: f32,
    pub strength: f32,
}

#[derive(Resource)]
pub struct EnemyManager {
    pub list: Vec<Enemy>,
}

impl EnemyManager {
    fn get_spawn_weights(&self, day: u32, time: f32) -> Vec<f32> {
        self.list
            .iter()
            .map(|&enemy| {
                if enemy.strength <= day as f32 {
                    // Enemies with less strength than the day can still
                    // spawn but with linearly decreasing probabilities
                    enemy.strength / day as f32
                } else {
                    // Exponentially decrease probability after day. The decay is less
                    // steep as time progresses, increasing the probability of stronger
                    // enemies over time
                    (-BETA * (1. - time / NIGHT_DURATION) * (enemy.strength - day as f32).powi(3))
                        .exp()
                }
            })
            .collect()
    }

    pub fn choose_enemy(&self, day: u32, time: f32) -> Option<&Enemy> {
        if rand::random::<f32>() > NO_SPAWN_START - NO_SPAWN_STEP * (day as f32 - 1.) {
            let weights = self.get_spawn_weights(day, time);
            let dist = WeightedIndex::new(&weights).unwrap();
            let index = dist.sample(&mut rand::thread_rng());
            Some(self.list.get(index).unwrap())
        } else {
            None
        }
    }
}

impl Default for EnemyManager {
    fn default() -> Self {
        let enemies = vec![
            Enemy {
                name: "Dartling",
                image: "enemy/dartling.png",
                max_health: 50.,
                health: 50.,
                dim: Vec2::new(40., 60.),
                size: Size::Medium,
                armor: 0.,
                speed: 5.,
                can_fly: false,
                damage: 10.,
                strength: 1.0,
            },
            Enemy {
                name: "Skitterling",
                image: "enemy/skitterling.png",
                max_health: 20.,
                health: 20.,
                dim: Vec2::new(25., 35.),
                size: Size::Small,
                armor: 0.,
                speed: 12.,
                can_fly: false,
                damage: 5.,
                strength: 1.0,
            },
            Enemy {
                name: "Shellback",
                image: "enemy/shellback.png",
                max_health: 50.,
                health: 50.,
                dim: Vec2::new(60., 80.),
                size: Size::Medium,
                armor: 2.,
                speed: 3.,
                can_fly: false,
                damage: 20.,
                strength: 2.0,
            },
            Enemy {
                name: "Quickstrike",
                image: "enemy/quickstrike.png",
                max_health: 40.,
                health: 40.,
                dim: Vec2::new(35., 45.),
                size: Size::Small,
                armor: 1.,
                speed: 10.,
                can_fly: false,
                damage: 10.,
                strength: 2.0,
            },
            Enemy {
                name: "Chiton",
                image: "enemy/chiton.png",
                max_health: 60.,
                health: 60.,
                dim: Vec2::new(40., 60.),
                size: Size::Medium,
                armor: 0.,
                speed: 7.,
                can_fly: false,
                damage: 15.,
                strength: 2.0,
            },
            Enemy {
                name: "Thornbiter",
                image: "enemy/thornbiter.png",
                max_health: 40.,
                health: 40.,
                dim: Vec2::new(45., 65.),
                size: Size::Medium,
                armor: 5.,
                speed: 5.,
                can_fly: false,
                damage: 20.,
                strength: 3.0,
            },
            Enemy {
                name: "Needler",
                image: "enemy/needler.png",
                max_health: 20.,
                health: 20.,
                dim: Vec2::new(45., 65.),
                size: Size::Medium,
                armor: 0.,
                speed: 15.,
                can_fly: true,
                damage: 10.,
                strength: 3.0,
            },
            Enemy {
                name: "Blightcraw",
                image: "enemy/blightcraw.png",
                max_health: 80.,
                health: 80.,
                dim: Vec2::new(35., 45.),
                size: Size::Small,
                armor: 2.,
                speed: 10.,
                can_fly: false,
                damage: 25.,
                strength: 4.0,
            },
            Enemy {
                name: "Shellfist",
                image: "enemy/shellfist.png",
                max_health: 120.,
                health: 120.,
                dim: Vec2::new(80., 100.),
                size: Size::Large,
                armor: 7.,
                speed: 2.,
                can_fly: false,
                damage: 50.,
                strength: 5.0,
            },
            Enemy {
                name: "Shellwarden",
                image: "enemy/shellwarden.png",
                max_health: 200.,
                health: 200.,
                dim: Vec2::new(80., 100.),
                size: Size::Large,
                armor: 4.,
                speed: 2.,
                can_fly: false,
                damage: 50.,
                strength: 5.0,
            },
            Enemy {
                name: "Hiveborn",
                image: "enemy/hiveborn.png",
                max_health: 30.,
                health: 30.,
                dim: Vec2::new(45., 45.),
                size: Size::Medium,
                armor: 0.,
                speed: 10.,
                can_fly: true,
                damage: 10.,
                strength: 5.0,
            },
            Enemy {
                name: "Crawler",
                image: "enemy/crawler.png",
                max_health: 55.,
                health: 55.,
                dim: Vec2::new(55., 55.),
                size: Size::Medium,
                armor: 0.,
                speed: 20.,
                can_fly: false,
                damage: 20.,
                strength: 6.0,
            },
            Enemy {
                name: "Carapacebreaker",
                image: "enemy/carapacebreaker.png",
                max_health: 30.,
                health: 30.,
                dim: Vec2::new(30., 45.),
                size: Size::Small,
                armor: 5.,
                speed: 15.,
                can_fly: false,
                damage: 50.,
                strength: 6.0,
            },
            Enemy {
                name: "Dartmite",
                image: "enemy/dartmite.png",
                max_health: 60.,
                health: 60.,
                dim: Vec2::new(40., 50.),
                size: Size::Medium,
                armor: 5.,
                speed: 10.,
                can_fly: true,
                damage: 15.,
                strength: 7.0,
            },
            Enemy {
                name: "Nestling",
                image: "enemy/nestling.png",
                max_health: 40.,
                health: 40.,
                dim: Vec2::new(40., 55.),
                size: Size::Medium,
                armor: 2.,
                speed: 25.,
                can_fly: false,
                damage: 15.,
                strength: 7.0,
            },
            Enemy {
                name: "Gargantula",
                image: "enemy/gargantula.png",
                max_health: 160.,
                health: 160.,
                dim: Vec2::new(120., 140.),
                size: Size::Large,
                armor: 7.,
                speed: 5.,
                can_fly: false,
                damage: 60.,
                strength: 8.0,
            },
            Enemy {
                name: "Ironclaw",
                image: "enemy/ironclaw.png",
                max_health: 150.,
                health: 150.,
                dim: Vec2::new(90., 90.),
                size: Size::Large,
                armor: 4.,
                speed: 15.,
                can_fly: false,
                damage: 50.,
                strength: 9.0,
            },
            Enemy {
                name: "Ironcarapace",
                image: "enemy/ironcarapace.png",
                max_health: 500.,
                health: 500.,
                dim: Vec2::new(200., 200.),
                size: Size::Huge,
                armor: 20.,
                speed: 1.,
                can_fly: false,
                damage: 80.,
                strength: 10.0,
            },
        ];

        Self { list: enemies }
    }
}
