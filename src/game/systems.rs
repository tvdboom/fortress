use crate::game::components::PauseWrapper;
use crate::game::resources::{Player, NightStats};
use crate::game::weapon::components::WeaponSettings;
use crate::game::{AppState, GameState};
use bevy::prelude::*;

pub fn new_game(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(Player::default());
    commands.insert_resource(WeaponSettings::default());
    commands.insert_resource(NightStats::default());
    next_state.set(GameState::Running);
}

pub fn start_game(mut commands: Commands, player: Res<Player>) {
    commands.insert_resource(NightStats {day: player.day, ..default()})
}

pub fn pause_game(
    mut vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut night_stats: ResMut<NightStats>,
) {
    night_stats.timer.pause();
    *vis_q.single_mut() = Visibility::Visible;
}

pub fn unpause_game(
    mut vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut night_stats: ResMut<NightStats>,
) {
    // PauseWrapper not yet spawned at first iteration
    if let Ok(mut e) = vis_q.get_single_mut() {
        night_stats.timer.unpause();
        *e = Visibility::Hidden;
    }
}

pub fn check_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if keyboard.just_pressed(KeyCode::KeyN) {
            next_app_state.set(AppState::StartGame);
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            std::process::exit(0);
        }
    }

    if keyboard.just_pressed(KeyCode::Space) && *app_state.get() == AppState::Game {
        match game_state.get() {
            GameState::Running => next_game_state.set(GameState::Paused),
            GameState::Paused => next_game_state.set(GameState::Running),
        }
    }
}
