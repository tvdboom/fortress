use crate::constants::{GAME_SPEED_STEP, MAX_GAME_SPEED};
use crate::game::map::components::PauseWrapper;
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::WeaponManager;
use crate::game::{AppState, GameState};
use bevy::prelude::*;

pub fn new_game(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(Player::init());
    commands.insert_resource(WeaponManager::default());
    commands.insert_resource(NightStats::default());
    next_state.set(GameState::Running);
}

pub fn start_night(mut commands: Commands, player: Res<Player>) {
    commands.insert_resource(NightStats {
        day: player.day,
        ..default()
    })
}

pub fn end_night(mut player: ResMut<Player>, night_stats: Res<NightStats>) {
    player
        .stats
        .entry(night_stats.day)
        .or_insert(night_stats.clone());
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
    mut game_settings: ResMut<GameSettings>,
    mut night_stats: ResMut<NightStats>,
) {
    // PauseWrapper not yet spawned at first iteration
    if let Ok(mut e) = vis_q.get_single_mut() {
        night_stats.timer.unpause();
        if game_settings.speed == 0. {
            game_settings.speed = 1.;
        }
        *e = Visibility::Hidden;
    }
}

pub fn check_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    mut game_settings: ResMut<GameSettings>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        game_settings.enemy_info = !game_settings.enemy_info;
    }

    if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if keyboard.just_pressed(KeyCode::KeyN) {
            next_app_state.set(AppState::StartGame);
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            std::process::exit(0);
        }

        if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            if keyboard.just_pressed(KeyCode::ArrowUp) {
                player.day += 1;
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) && player.day > 1 {
                player.day -= 1;
            }
        }
    }

    if *app_state.get() == AppState::Night {
        if keyboard.just_pressed(KeyCode::Space) {
            match game_state.get() {
                GameState::Running => next_game_state.set(GameState::Paused),
                GameState::Paused => next_game_state.set(GameState::Running),
            }
        }

        if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            if keyboard.just_pressed(KeyCode::ArrowLeft) && game_settings.speed >= GAME_SPEED_STEP {
                game_settings.speed -= GAME_SPEED_STEP;
                if game_settings.speed == 0. {
                    next_game_state.set(GameState::Paused);
                }
            }
            if keyboard.just_pressed(KeyCode::ArrowRight) && game_settings.speed <= MAX_GAME_SPEED {
                game_settings.speed += GAME_SPEED_STEP;
                if game_settings.speed == GAME_SPEED_STEP {
                    next_game_state.set(GameState::Running);
                }
            }
        }
    }
}
