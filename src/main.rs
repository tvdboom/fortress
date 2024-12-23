mod game;

use crate::game::GamePlugin;

use bevy::prelude::*;

const TITLE: &str = "Fortress";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Buy,
    #[default]
    MainMenu,
    Game,
    GameOver,
}
