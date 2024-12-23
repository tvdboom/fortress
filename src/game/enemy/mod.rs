pub mod components;
mod resources;
mod systems;

use std::time::Duration;
use crate::AppState;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::game::enemy::systems::spawn_enemies;
use super::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_enemies.run_if(on_timer(Duration::from_secs(1))));
        // .add_systems(
        //     (
        //         enemy_movement,
        //         update_enemy_direction,
        //         confine_enemy_movement,
        //         tick_enemy_spawn_timer,
        //         spawn_enemies_over_time,
        //     )
        //         .in_set(OnUpdate(AppState::Game))
        //         .in_set(OnUpdate(SimulationState::Running)),
        // )
        // // Exit State Systems
        // .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
