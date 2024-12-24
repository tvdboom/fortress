use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Enemy {
    pub name: String,
    pub image: String,
    pub max_health: u32,
    pub health: u32,
    pub size: f32,
    pub speed: f32,
    pub damage: u32,
}

impl Enemy {
    pub fn walker() -> Self {
        Self {
            name: "Walker".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 100,
            health: 100,
            size: 50.,
            speed: 10.,
            damage: 1,
        }
    }

    pub fn runner() -> Self {
        Self {
            name: "Runner".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 100,
            health: 100,
            size: 50.,
            speed: 20.,
            damage: 1,
        }
    }

    pub fn ogre() -> Self {
        Self {
            name: "Ogre".to_string(),
            image: "enemy/walker.png".to_string(),
            max_health: 200,
            health: 100,
            size: 70.,
            speed: 5.,
            damage: 2,
        }
    }
}
