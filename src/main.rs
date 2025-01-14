pub mod constants;
mod game;
mod messages;
mod systems;
mod utils;

use crate::game::GamePlugin;
use constants::{SIZE, TITLE};

use crate::messages::MessagesPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // Prevents blurry sprites
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
        .add_plugins(MessagesPlugin::default())
        .add_plugins(GamePlugin);

        #[cfg(target_os = "windows")]
        app.add_systems(Startup, systems::set_window_icon);

        app.run();
}
