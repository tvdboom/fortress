pub mod components;
mod systems;
pub mod utils;

use super::{AppState, GameState};
use crate::game::enemy::systems::*;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_enemies, move_enemies)
                .run_if(in_state(AppState::Night).and(in_state(GameState::Running))),
        );
    }
}
