mod enemy;
mod map;
mod systems;

use crate::game::enemy::EnemyPlugin;
use crate::game::map::MapPlugin;
use crate::game::systems::pause_game;
use crate::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, EnemyPlugin))
            .init_state::<GameState>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Finished,
}
