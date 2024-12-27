mod game;
mod resources;
mod systems;
use crate::game::map::constants::SIZE;
use crate::game::GamePlugin;

use crate::resources::Player;
use crate::systems::set_window_icon;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;

const TITLE: &str = "Fortress";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: TITLE.into(),
                        resolution: WindowResolution::new(SIZE.x, SIZE.y),
                        resizable: false,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, set_window_icon)
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
