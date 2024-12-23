mod enemy;
mod map;
mod resources;
mod systems;

use crate::game::enemy::EnemyPlugin;
use crate::game::map::MapPlugin;
use bevy::prelude::*;
use crate::game::resources::WaveStats;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, EnemyPlugin))
            .init_state::<GameState>()
            .init_resource::<WaveStats>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Finished,
}
