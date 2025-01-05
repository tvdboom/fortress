pub mod components;
pub mod spawn;
mod systems;

use std::time::Duration;

use super::{AppState, GameState};
use crate::game::enemy::spawn::EnemySpawner;
use crate::game::enemy::systems::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawner>().add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_millis(300))),
                move_enemies,
                update_enemy_health_bars,
            )
                .run_if(in_state(AppState::Night))
                .run_if(in_state(GameState::Running)),
        );
    }
}
