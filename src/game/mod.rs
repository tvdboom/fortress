pub mod components;
pub mod enemy;
pub mod map;
pub mod resources;
pub mod systems;
pub mod weapon;

use crate::game::enemy::EnemyPlugin;
use crate::game::map::MapPlugin;
use crate::game::resources::WaveStats;
use crate::game::systems::toggle_pause;
use crate::game::weapon::WeaponPlugin;
use crate::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, EnemyPlugin, WeaponPlugin))
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::Game)))
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
