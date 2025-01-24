use crate::constants::*;
use crate::game::map::components::{FogOfWar, PauseWrapper};
use crate::game::resources::*;
use crate::game::weapon::components::WeaponManager;
use crate::game::{AppState, AudioState, GameState};
use crate::messages::Messages;
use crate::utils::{load_game, save_game};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use rand::prelude::*;
use rand_distr::Normal;

pub fn new_game(
    mut commands: Commands,
    mut fow_q: Query<&mut Transform, With<FogOfWar>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(Player::init());
    commands.insert_resource(WeaponManager::default());
    commands.insert_resource(NightStats::default());

    // Reset the fow's position
    if let Ok(mut transform) = fow_q.get_single_mut() {
        transform.translation.y = SIZE.y * 0.5 - MENU_PANEL_SIZE.y - FOW_SIZE.y * 0.5;
    }

    next_state.set(GameState::Running);
}

pub fn start_night(mut commands: Commands, player: Res<Player>) {
    commands.insert_resource(NightStats {
        day: player.day,
        ..default()
    });
}

pub fn end_night(mut player: ResMut<Player>, night_stats: Res<NightStats>) {
    player.stats.entry(night_stats.day).or_insert(NightInfo {
        day: night_stats.day,
        population: night_stats.population.clone(),
        resources: night_stats.resources.clone(),
        enemies: night_stats.enemies.clone(),
    });
}

pub fn start_day(
    mut player: ResMut<Player>,
    mut messages: ResMut<Messages>,
    mut game_settings: ResMut<GameSettings>,
) {
    if !game_settings.just_loaded {
        player.day += 1;

        // Increase population
        let dist = Normal::new(
            (POPULATION_MEAN_INCREASE * player.day) as f32,
            (POPULATION_STD_INCREASE * player.day) as f32,
        )
        .unwrap();

        let new_population = dist.sample(&mut thread_rng()) as u32;
        player.population.idle += new_population;
        messages.info(format!("Population increased by {}.", new_population));

        let new_resources = player.new_resources();
        player.resources += &new_resources;

        if let Some(ref mut expedition) = &mut player.expedition {
            expedition.update();
        }
    } else {
        game_settings.just_loaded = false;
    }

    game_settings.day_tab = DayTabs::Overview;
}

pub fn pause_game(
    mut vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut night_stats: ResMut<NightStats>,
    audio: Res<Audio>,
) {
    night_stats.timer.pause();
    *vis_q.single_mut() = Visibility::Visible;
    audio.pause();
}

pub fn unpause_game(
    mut vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut game_settings: ResMut<GameSettings>,
    mut night_stats: ResMut<NightStats>,
    audio: Res<Audio>,
) {
    // PauseWrapper not yet spawned at first iteration
    if let Ok(mut e) = vis_q.get_single_mut() {
        night_stats.timer.unpause();
        if game_settings.speed == 0. {
            game_settings.speed = 1.;
        }
        *e = Visibility::Hidden;
        audio.resume();
    }
}

pub fn stop_audio(
    mut game_settings: ResMut<GameSettings>,
    mut messages: ResMut<Messages>,
    audio: Res<Audio>,
) {
    audio.stop();
    game_settings.audio = false;
    messages.info("Audio disabled.");
}

pub fn play_audio(mut game_settings: ResMut<GameSettings>) {
    game_settings.audio = true;
}

pub fn check_keys(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    mut game_settings: ResMut<GameSettings>,
    mut messages: ResMut<Messages>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_audio_state: ResMut<NextState<AudioState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        game_settings.enemy_info = !game_settings.enemy_info;
    }

    if keyboard.just_pressed(KeyCode::KeyA) {
        match game_settings.audio {
            true => next_audio_state.set(AudioState::Stopped),
            false => next_audio_state.set(AudioState::Playing),
        }
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        match *app_state.get() {
            AppState::StartGame => next_app_state.set(AppState::Night),
            AppState::Day => {
                if player.expedition.is_some()
                    && !matches!(
                        player.expedition.as_ref().unwrap().status,
                        ExpeditionStatus::Ongoing
                    )
                {
                    player.resolve_expedition();
                } else {
                    if player.population.idle > 0 {
                        messages.error("You have idle population!");
                    } else {
                        next_app_state.set(AppState::Night);
                    }
                }
            }
            AppState::GameOver => next_app_state.set(AppState::StartGame),
            _ => (),
        }
    }

    if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if keyboard.just_pressed(KeyCode::KeyN) {
            next_app_state.set(AppState::StartGame);
        } else if keyboard.just_pressed(KeyCode::KeyL) {
            load_game(
                &mut commands,
                &game_settings,
                &mut next_app_state,
                &mut messages,
            );
        } else if keyboard.just_pressed(KeyCode::KeyS) {
            save_game(&player, &game_settings, &mut messages);
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            std::process::exit(0);
        }

        if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            if keyboard.just_pressed(KeyCode::ArrowUp) {
                player.day += 1;
                player.resources += 2000.;
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) && player.day > 1 {
                player.day -= 1;
            }
            if keyboard.just_pressed(KeyCode::ArrowRight) {
                next_app_state.set(AppState::Day);
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
