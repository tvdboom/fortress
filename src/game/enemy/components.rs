use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyHealthWrapper;

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Component, Clone)]
pub struct Enemy {
    pub name: String,
    pub image: String,
    pub max_health: u32,
    pub health: u32,
    pub size: (f32, f32),
    pub speed: f32,
    pub damage: u32,
}

impl Enemy {
    pub fn walker() -> Self {
        Self {
            name: "Walker".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 100,
            health: 20,
            size: (50., 50.),
            speed: 5.,
            damage: 1,
        }
    }

    pub fn runner() -> Self {
        Self {
            name: "Runner".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 100,
            health: 10,
            size: (50., 50.),
            speed: 10.,
            damage: 1,
        }
    }

    pub fn ogre() -> Self {
        Self {
            name: "Ogre".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 200,
            health: 50,
            size: (70., 70.),
            speed: 2.5,
            damage: 2,
        }
    }
}
