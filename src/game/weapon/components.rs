use crate::resources::Resources;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Bullet {
    pub image: String,
    pub size: Vec2,
    pub speed: f32,
    pub angle: f32,
    pub damage: u32,
    pub max_distance: f32, // 0-100 as percentage of map's height
    pub distance: f32,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: String,
    pub image: String,
    pub size: Vec2,
    pub price: Resources,
    pub fire_rate: Timer,
    pub fire_cost: Resources,
    pub bullet: Bullet,
}

impl Weapon {
    pub fn sentry_gun() -> Self {
        Self {
            name: "Sentry gun".to_string(),
            image: "weapon/sentry-gun.png".to_string(),
            size: Vec2::new(110., 110.),
            price: Resources {
                materials: 100,
                ..default()
            },
            fire_rate: Timer::from_seconds(1., TimerMode::Repeating),
            fire_cost: Resources {
                bullets: 1,
                ..default()
            },
            bullet: Bullet {
                image: "weapon/bullet.png".to_string(),
                size: Vec2::new(30., 30.),
                speed: 50.,
                angle: 0.,
                damage: 5,
                max_distance: 70.,
                distance: 0.,
            },
        }
    }
}
