pub(crate) mod components;
mod systems;

use crate::game::map::systems::{setup, text_update};
use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                text_update
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running)),
            );
    }
}
