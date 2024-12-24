use crate::resources::Resources;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct SentryGun {
    pub name: String,
    pub size: f32,
    pub price: Resources,
    pub damage: u32,
    pub range: u32,     // 0-100 as percentage of the map
    pub fire_rate: u32, // Every X calls to shoot timer
    pub fire_cost: Resources,
}

impl Default for SentryGun {
    fn default() -> Self {
        Self {
            name: "Sentry Gun".to_string(),
            size: 50.,
            price: Resources {
                materials: 100,
                ..default()
            },
            damage: 10,
            range: 50,
            fire_rate: 5,
            fire_cost: Resources {
                bullets: 1,
                ..default()
            },
        }
    }
}
