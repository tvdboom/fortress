use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Component, Copy, Clone)]
pub struct Enemy {
    pub name: &'static str,
    pub image: &'static str,
    pub max_health: f32,
    pub health: f32,
    pub size: Vec2,
    pub armor: f32,
    pub speed: f32,
    pub can_fly: bool,
    pub damage: f32,
    pub strength: f32,
}
