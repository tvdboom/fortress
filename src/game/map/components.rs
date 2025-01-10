use crate::game::weapon::components::Explosion;
use bevy::prelude::*;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct FogOfWar;

#[derive(Component)]
pub struct PauseWrapper;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct AnimationComponent {
    pub timer: Timer,
    pub last_index: usize,
    pub explosion: Option<Explosion>,
}
