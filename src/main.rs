mod game;
use crate::game::GamePlugin;

use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};

const TITLE: &str = "Fortress";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resizable: false,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .init_state::<AppState>()
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    BuyMenu,
    #[default]
    Game,
    GameOver,
    MainMenu,
}
