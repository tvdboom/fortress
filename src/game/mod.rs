mod enemy;
mod map;
mod systems;

use crate::game::map::MapPlugin;
use crate::game::systems::pause_game;
use crate::AppState;
use bevy::prelude::*;
use crate::game::enemy::EnemyPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, EnemyPlugin));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
