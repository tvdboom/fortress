use crate::game::components::PauseWrapper;
use crate::game::resources::{Player, WaveStats};
use crate::game::{AppState, GameState};
use bevy::prelude::*;

pub fn new_game(
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(Player::default());
    next_game_state.set(GameState::Running);
    next_app_state.set(AppState::StartGame);
}

pub fn start_game(mut commands: Commands) {
    commands.insert_resource(WaveStats::default())
}

pub fn pause_game(
    mut vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    *vis_q.single_mut() = Visibility::Visible;
    next_state.set(GameState::Paused);
}

pub fn resume_game(
    mut vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    *vis_q.single_mut() = Visibility::Hidden;
    next_state.set(GameState::Running);
}

pub fn toggle_pause(
    vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameState::Running => pause_game(vis_q, next_state),
            GameState::Paused => resume_game(vis_q, next_state),
        }
    }
}
