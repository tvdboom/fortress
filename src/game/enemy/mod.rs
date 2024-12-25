pub mod components;
mod systems;

use crate::AppState;
use std::time::Duration;

use super::GameState;
use crate::game::enemy::systems::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_millis(300))),
                move_enemies,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
    }
}
