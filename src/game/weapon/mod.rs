pub mod components;
mod systems;

use super::{AppState, GameState};
use crate::game::weapon::systems::*;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartGame), draw_weapons)
            .add_systems(OnEnter(AppState::Game), draw_weapons)
            .add_systems(
                Update,
                (spawn_bullets, move_bullets)
                    .run_if(in_state(AppState::Game).and(in_state(GameState::Running))),
            );
    }
}
