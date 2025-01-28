use crate::game::weapon::components::Explosion;
use bevy::prelude::*;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct FogOfWar;

#[derive(Component)]
pub struct Pause;

#[derive(Component)]
pub struct AnimationComponent {
    pub timer: Timer,
    pub last_index: usize,
    pub explosion: Option<Explosion>,
}
