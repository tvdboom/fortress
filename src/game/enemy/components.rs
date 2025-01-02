use bevy::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnemyType {
    Walker,
    Runner,
    Ogre,
    ArmoredOgre,
    Dragon,
}

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Component, Clone)]
pub struct Enemy {
    pub name: EnemyType,
    pub image: String,
    pub max_health: u32,
    pub health: u32,
    pub size: Vec2,
    pub speed: f32,
    pub damage: f32,
}

impl Enemy {
    pub fn walker() -> Self {
        Self {
            name: EnemyType::Walker,
            image: "enemy/walker.png".to_string(),
            max_health: 20,
            health: 20,
            size: Vec2::new(50., 50.),
            speed: 5.,
            damage: 10.,
        }
    }

    pub fn runner() -> Self {
        Self {
            name: EnemyType::Runner,
            image: "enemy/runner.png".to_string(),
            max_health: 10,
            health: 10,
            size: Vec2::new(50., 50.),
            speed: 10.,
            damage: 10.,
        }
    }

    pub fn ogre() -> Self {
        Self {
            name: EnemyType::Ogre,
            image: "enemy/ogre.png".to_string(),
            max_health: 50,
            health: 50,
            size: Vec2::new(70., 70.),
            speed: 2.5,
            damage: 20.,
        }
    }

    pub fn armored_ogre() -> Self {
        Self {
            name: EnemyType::ArmoredOgre,
            image: "enemy/ogre-armor.png".to_string(),
            max_health: 80,
            health: 80,
            size: Vec2::new(70., 70.),
            speed: 2.5,
            damage: 30.,
        }
    }

    pub fn dragon() -> Self {
        Self {
            name: EnemyType::Dragon,
            image: "enemy/dragon.png".to_string(),
            max_health: 150,
            health: 150,
            size: Vec2::new(90., 90.),
            speed: 7.5,
            damage: 50.,
        }
    }
}
