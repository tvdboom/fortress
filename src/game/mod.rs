pub mod components;
pub mod enemy;
pub mod map;
pub mod resources;
pub mod systems;
pub mod weapon;

use crate::game::enemy::EnemyPlugin;
use crate::game::map::MapPlugin;
use crate::game::systems::*;
use crate::game::weapon::WeaponPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, EnemyPlugin, WeaponPlugin))
            .add_systems(OnEnter(AppState::StartGame), new_game)
            .add_systems(OnEnter(AppState::Game), start_game)
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::Game)))
            .init_state::<AppState>()
            .init_state::<GameState>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    BuyMenu,
    Game,
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
