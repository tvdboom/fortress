pub mod components;
pub mod constants;
pub mod systems;

use crate::game::map::systems::*;
use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (
                    resources_panel,
                    weapons_panel,
                    map_update.run_if(in_state(GameState::Running)),
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
