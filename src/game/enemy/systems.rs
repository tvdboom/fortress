use super::components::*;
use super::resources::*;
use crate::game::enemy::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.get_single() {
        let random_x = random::<f32>() * window.width();

        commands.spawn((
            Sprite {
                image: asset_server.load("enemy/walker.png"),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            Walker::default(),
            Transform {
                translation: Vec3::new(random_x, window.height(), 2.0),
                ..default()
            },
        ));
    }
}
