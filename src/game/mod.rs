pub mod components;
pub mod enemy;
pub mod map;
pub mod resources;
pub mod systems;
pub mod weapon;

use crate::game::enemy::EnemyPlugin;
use crate::game::map::MapPlugin;
use crate::game::resources::{GameSettings, NightStats};
use crate::game::systems::*;
use crate::game::weapon::WeaponPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, EnemyPlugin, WeaponPlugin))
            .add_systems(OnEnter(AppState::StartGame), new_game)
            .add_systems(OnEnter(AppState::Night), start_night)
            .add_systems(OnExit(AppState::Night), end_night)
            .add_systems(OnEnter(GameState::Paused), pause_game)
            .add_systems(OnEnter(GameState::Running), unpause_game)
            .add_systems(Update, check_keys)
            .init_state::<AppState>()
            .init_state::<GameState>()
            .init_resource::<GameSettings>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Day,
    EndNight,
    Night,
    GameOver,
    #[default]
    StartGame,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
