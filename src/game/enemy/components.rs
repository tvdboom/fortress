use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Component, Clone)]
pub struct Enemy {
    pub name: String,
    pub image: String,
    pub max_health: u32,
    pub health: u32,
    pub size: Vec2,
    pub speed: f32,
    pub damage: u32,
}

impl Enemy {
    pub fn walker() -> Self {
        Self {
            name: "Walker".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 20,
            health: 20,
            size: Vec2::new(50., 50.),
            speed: 5.,
            damage: 1,
        }
    }

    pub fn runner() -> Self {
        Self {
            name: "Runner".to_string(),
            image: "enemy/runner.png".to_string(),
            max_health: 10,
            health: 10,
            size: Vec2::new(50., 50.),
            speed: 10.,
            damage: 1,
        }
    }

    pub fn dragon() -> Self {
        Self {
            name: "Dragon".to_string(),
            image: "enemy/dragon.png".to_string(),
            max_health: 50,
            health: 50,
            size: Vec2::new(70., 70.),
            speed: 2.5,
            damage: 2,
        }
    }
}
