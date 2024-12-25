mod game;
mod resources;
mod systems;

use crate::game::GamePlugin;

use crate::resources::Player;
use bevy::prelude::*;
use bevy::window::WindowResolution;

const TITLE: &str = "Fortress";

// We fix the window size to avoid issues with resizing
const WIDTH: f32 = 1280.;
const HEIGHT: f32 = 800.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: TITLE.into(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        resizable: false,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .init_state::<AppState>()
        .init_resource::<Player>()
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
