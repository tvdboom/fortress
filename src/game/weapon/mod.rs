pub mod components;
mod systems;

use super::{AppState, GameState};
use crate::game::weapon::systems::*;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Night), spawn_weapons)
            .add_systems(
                Update,
                (spawn_bullets, move_bullets, update_fence_resources)
                    .run_if(in_state(AppState::Night).and(in_state(GameState::Running))),
            );
    }
}
