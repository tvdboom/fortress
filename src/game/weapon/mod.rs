pub mod components;
mod systems;

use super::{AppState, GameState};
use crate::game::weapon::components::*;
use crate::game::weapon::systems::*;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponSettings>().add_systems(
            Update,
            (spawn_bullets, move_bullets)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
    }
}
