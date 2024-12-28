pub mod components;
pub mod constants;
pub mod systems;

use crate::game::map::systems::*;
use crate::game::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (set_style, setup_map))
            .add_systems(
                Update,
                (
                    menu_panel,
                    resources_panel,
                    weapons_panel,
                    start_end_game_panel
                        .run_if(in_state(AppState::StartGame).or(in_state(AppState::GameOver))),
                ),
            );
    }
}
