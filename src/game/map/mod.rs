pub mod components;
pub mod systems;
pub mod utils;

use crate::game::map::systems::*;
use crate::game::{AppState, GameState};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (set_style, draw_map))
            .add_systems(OnEnter(AppState::StartGame), clear_map)
            .add_systems(OnEnter(AppState::Day), clear_map)
            .add_systems(
                Update,
                (
                    (
                        weapons_panel,
                        menu_panel,
                        resources_panel,
                        enemy_info_panel,
                        run_animations,
                    )
                        .chain(),
                    info_panel.run_if(not(in_state(AppState::Night))),
                    update_game.run_if(in_state(AppState::Night).and(in_state(GameState::Running))),
                ),
            );
    }
}
