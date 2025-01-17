use crate::constants::{
    GAME_SPEED_STEP, MAX_GAME_SPEED, POPULATION_MEAN_INCREASE, POPULATION_STD_INCREASE,
    RESOURCE_FACTOR,
};
use crate::game::map::components::PauseWrapper;
use crate::game::resources::{
    DayTabs, GameSettings, NightStats, Player, Resources, TechnologyName,
};
use crate::game::weapon::components::WeaponManager;
use crate::game::{AppState, GameState};
use crate::messages::Messages;
use bevy::prelude::*;
use rand::prelude::*;
use rand_distr::Normal;

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
    });
}

pub fn end_night(mut player: ResMut<Player>, night_stats: Res<NightStats>) {
    player
        .stats
        .entry(night_stats.day)
        .or_insert(night_stats.clone());
}

pub fn start_day(
    mut player: ResMut<Player>,
    mut messages: ResMut<Messages>,
    mut game_settings: ResMut<GameSettings>,
) {
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

    // Increase resources
    let population = player.population.clone();
    let productivity = if player.has_tech(TechnologyName::Productivity) {
        1.5
    } else {
        1.
    };

    player.resources += &Resources {
        bullets: (population.armorer * RESOURCE_FACTOR) as f32 * productivity,
        gasoline: (population.refiner * RESOURCE_FACTOR) as f32 * productivity,
        materials: (population.constructor * RESOURCE_FACTOR) as f32 * productivity,
        technology: (population.scientist * RESOURCE_FACTOR) as f32 * productivity,
    };

    // Resolve expeditions
    if let Some(ref mut expedition) = &mut player.expedition {
        player.expedition = expedition.check(&mut player);
    }

    game_settings.day_tab = DayTabs::Overview;
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

    if keyboard.just_pressed(KeyCode::Enter) {
        match *app_state.get() {
            AppState::StartGame => next_app_state.set(AppState::Night),
            AppState::Day => next_app_state.set(AppState::Night),
            AppState::GameOver => next_app_state.set(AppState::StartGame),
            _ => (),
        }
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
