pub mod components;
mod systems;

use super::{AppState, GameState};
use crate::game::weapon::components::*;
use crate::game::weapon::systems::*;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_weapons,
                (spawn_bullets, move_bullets)
                    .run_if(in_state(AppState::Game).and(in_state(GameState::Running))),
            ),
        )
        .init_resource::<WeaponSettings>();
    }
}
