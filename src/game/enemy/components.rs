use crate::constants::*;
use bevy::prelude::*;
use bevy::prelude::{Resource, Vec2};
use rand::distributions::{Distribution, WeightedIndex};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
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
    pub flies: bool,
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
        if rand::random::<f32>() > (NO_SPAWN_START - NO_SPAWN_STEP * day as f32).max(0.1) {
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
                speed: 0.05 * MAP_SIZE.y,
                flies: false,
                damage: 10.,
                strength: 1.,
            },
            Enemy {
                name: "Skitterling",
                image: "enemy/skitterling.png",
                max_health: 20.,
                health: 20.,
                dim: Vec2::new(25., 35.),
                size: Size::Small,
                armor: 0.,
                speed: 0.12 * MAP_SIZE.y,
                flies: false,
                damage: 5.,
                strength: 1.,
            },
            Enemy {
                name: "Shellback",
                image: "enemy/shellback.png",
                max_health: 50.,
                health: 50.,
                dim: Vec2::new(60., 80.),
                size: Size::Medium,
                armor: 2.,
                speed: 0.03 * MAP_SIZE.y,
                flies: false,
                damage: 20.,
                strength: 2.,
            },
            Enemy {
                name: "Grub",
                image: "enemy/grub.png",
                max_health: 40.,
                health: 40.,
                dim: Vec2::new(35., 35.),
                size: Size::Small,
                armor: 0.,
                speed: 0.1 * MAP_SIZE.y,
                flies: false,
                damage: 20.,
                strength: 3.,
            },
            Enemy {
                name: "Quickstrike",
                image: "enemy/quickstrike.png",
                max_health: 50.,
                health: 50.,
                dim: Vec2::new(35., 45.),
                size: Size::Small,
                armor: 1.,
                speed: 0.1 * MAP_SIZE.y,
                flies: false,
                damage: 25.,
                strength: 4.,
            },
            Enemy {
                name: "Chiton",
                image: "enemy/chiton.png",
                max_health: 70.,
                health: 70.,
                dim: Vec2::new(40., 60.),
                size: Size::Medium,
                armor: 1.,
                speed: 0.07 * MAP_SIZE.y,
                flies: false,
                damage: 15.,
                strength: 5.,
            },
            Enemy {
                name: "Thornbiter",
                image: "enemy/thornbiter.png",
                max_health: 90.,
                health: 90.,
                dim: Vec2::new(45., 65.),
                size: Size::Medium,
                armor: 4.,
                speed: 0.05 * MAP_SIZE.y,
                flies: false,
                damage: 30.,
                strength: 6.,
            },
            Enemy {
                name: "Needler",
                image: "enemy/needler.png",
                max_health: 80.,
                health: 80.,
                dim: Vec2::new(45., 65.),
                size: Size::Medium,
                armor: 0.,
                speed: 0.15 * MAP_SIZE.y,
                flies: true,
                damage: 30.,
                strength: 7.,
            },
            Enemy {
                name: "Blightcraw",
                image: "enemy/blightcraw.png",
                max_health: 120.,
                health: 120.,
                dim: Vec2::new(35., 45.),
                size: Size::Small,
                armor: 4.,
                speed: 0.1 * MAP_SIZE.y,
                flies: false,
                damage: 55.,
                strength: 8.,
            },
            Enemy {
                name: "Shellfist",
                image: "enemy/shellfist.png",
                max_health: 200.,
                health: 200.,
                dim: Vec2::new(80., 100.),
                size: Size::Large,
                armor: 7.,
                speed: 0.04 * MAP_SIZE.y,
                flies: false,
                damage: 120.,
                strength: 9.,
            },
            Enemy {
                name: "Shellwarden",
                image: "enemy/shellwarden.png",
                max_health: 300.,
                health: 300.,
                dim: Vec2::new(80., 100.),
                size: Size::Large,
                armor: 6.,
                speed: 0.04 * MAP_SIZE.y,
                flies: false,
                damage: 140.,
                strength: 10.,
            },
            Enemy {
                name: "Hiveborn",
                image: "enemy/hiveborn.png",
                max_health: 80.,
                health: 80.,
                dim: Vec2::new(45., 45.),
                size: Size::Medium,
                armor: 1.,
                speed: 0.1 * MAP_SIZE.y,
                flies: true,
                damage: 45.,
                strength: 11.,
            },
            Enemy {
                name: "Hornet",
                image: "enemy/hornet.png",
                max_health: 145.,
                health: 145.,
                dim: Vec2::new(55., 55.),
                size: Size::Medium,
                armor: 0.,
                speed: 0.2 * MAP_SIZE.y,
                flies: false,
                damage: 65.,
                strength: 12.,
            },
            Enemy {
                name: "Crawler",
                image: "enemy/crawler.png",
                max_health: 155.,
                health: 155.,
                dim: Vec2::new(55., 55.),
                size: Size::Medium,
                armor: 6.,
                speed: 0.2 * MAP_SIZE.y,
                flies: false,
                damage: 75.,
                strength: 13.,
            },
            Enemy {
                name: "Breaker",
                image: "enemy/breaker.png",
                max_health: 300.,
                health: 300.,
                dim: Vec2::new(30., 45.),
                size: Size::Small,
                armor: 7.,
                speed: 0.15 * MAP_SIZE.y,
                flies: false,
                damage: 155.,
                strength: 14.,
            },
            Enemy {
                name: "Weevil",
                image: "enemy/weevil.png",
                max_health: 280.,
                health: 280.,
                dim: Vec2::new(40., 55.),
                size: Size::Medium,
                armor: 11.,
                speed: 0.1 * MAP_SIZE.y,
                flies: false,
                damage: 145.,
                strength: 15.,
            },
            Enemy {
                name: "Mothroach",
                image: "enemy/mothroach.png",
                max_health: 620.,
                health: 620.,
                dim: Vec2::new(45., 60.),
                size: Size::Medium,
                armor: 11.,
                speed: 0.07 * MAP_SIZE.y,
                flies: false,
                damage: 175.,
                strength: 16.,
            },
            Enemy {
                name: "Dartmite",
                image: "enemy/dartmite.png",
                max_health: 260.,
                health: 260.,
                dim: Vec2::new(40., 50.),
                size: Size::Medium,
                armor: 5.,
                speed: 0.05 * MAP_SIZE.y,
                flies: true,
                damage: 95.,
                strength: 17.,
            },
            Enemy {
                name: "Nestling",
                image: "enemy/nestling.png",
                max_health: 840.,
                health: 840.,
                dim: Vec2::new(40., 55.),
                size: Size::Medium,
                armor: 12.,
                speed: 0.25 * MAP_SIZE.y,
                flies: false,
                damage: 255.,
                strength: 18.,
            },
            Enemy {
                name: "Gargantula",
                image: "enemy/gargantula.png",
                max_health: 1260.,
                health: 1260.,
                dim: Vec2::new(120., 140.),
                size: Size::Large,
                armor: 15.,
                speed: 0.05 * MAP_SIZE.y,
                flies: false,
                damage: 360.,
                strength: 19.,
            },
            Enemy {
                name: "Ironclaw",
                image: "enemy/ironclaw.png",
                max_health: 1500.,
                health: 1500.,
                dim: Vec2::new(90., 90.),
                size: Size::Large,
                armor: 15.,
                speed: 0.15 * MAP_SIZE.y,
                flies: false,
                damage: 250.,
                strength: 20.,
            },
            Enemy {
                name: "Reaper",
                image: "enemy/reaper.png",
                max_health: 3500.,
                health: 3500.,
                dim: Vec2::new(200., 200.),
                size: Size::Huge,
                armor: 20.,
                speed: 0.1 * MAP_SIZE.y,
                flies: false,
                damage: 580.,
                strength: 21.,
            },
        ];

        Self { list: enemies }
    }
}
