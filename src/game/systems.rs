use bevy::prelude::*;

use crate::game::GameState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<GameState>>) {
    simulation_state_next_state.set(GameState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<GameState>>) {
    simulation_state_next_state.set(GameState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_state == GameState::Running {
            next_state.set(GameState::Paused);
            println!("Simulation Paused.");
        }
        if game_state == GameState::Paused {
            next_state.set(GameState::Running);
            println!("Simulation Running.");
        }
    }
}