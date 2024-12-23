use bevy::prelude::*;

use crate::game::GameState;

pub fn pause_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Paused);
}

pub fn resume_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Running);
}

// pub fn toggle_pause(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     game_state: Res<State<GameState>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         if game_state == GameState::Running {
//             next_state.set(GameState::Paused);
//             println!("Game Paused.");
//         }
//         if game_state == GameState::Paused {
//             next_state.set(GameState::Running);
//             println!("Game Running.");
//         }
//     }
// }
