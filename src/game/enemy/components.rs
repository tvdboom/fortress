use bevy::prelude::*;

#[derive(Component)]
pub struct Walker {
    max_health: u32,
    health: u32,
    size: f32,
    speed: u32,
    damage: u32,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            max_health: 100,
            health: 100,
            size: 50.0,
            speed: 10,
            damage: 10,
        }
    }
}
