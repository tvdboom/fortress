use bevy::prelude::*;
use std::any::type_name;

#[derive(Component)]
pub struct LifeBar;

#[derive(Component)]
pub struct LifeBarWrapper;

#[derive(Component, Clone)]
pub struct Walker {
    pub name: String,
    pub max_health: u32,
    pub health: u32,
    pub size: f32,
    pub speed: f32,
    pub damage: u32,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            name: type_name::<Self>().to_string(),
            max_health: 100,
            health: 100,
            size: 50.,
            speed: 10.,
            damage: 10,
        }
    }
}
