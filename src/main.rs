pub mod constants;
mod game;
mod messages;
mod systems;
mod utils;

use crate::game::enemy::components::EnemyManager;
use crate::game::GamePlugin;
use crate::messages::MessagesPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use constants::{SIZE, TITLE};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest()) // Prevents blurry sprites
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    mode: WindowMode::Windowed,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: WindowResolution::new(SIZE.x, SIZE.y),
                    resizable: false,
                    resize_constraints: WindowResizeConstraints {
                        min_width: SIZE.x,
                        min_height: SIZE.y,
                        max_width: f32::MAX,
                        max_height: f32::MAX,
                    },
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            // Disable asset meta loading since that fails on itch.io
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
    )
    .add_plugins((AudioPlugin, EguiPlugin))
    .add_plugins(MessagesPlugin::default())
    .add_plugins(GamePlugin)
    .init_resource::<EnemyManager>();

    #[cfg(target_os = "windows")]
    app.add_systems(Startup, systems::set_window_icon);

    app.run();
}
