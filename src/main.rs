mod game;
mod resources;

use crate::game::GamePlugin;

use crate::resources::Player;
use bevy::prelude::*;

const TITLE: &str = "Fortress";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: TITLE.into(),
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
