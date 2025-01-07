pub mod components;
mod systems;

use super::{AppState, GameState};
use crate::game::enemy::components::EnemyManager;
use crate::game::enemy::systems::*;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyManager>().add_systems(
            Update,
            (spawn_enemies, move_enemies, update_enemy_health_bars)
                .run_if(in_state(AppState::Night))
                .run_if(in_state(GameState::Running)),
        );
    }
}
