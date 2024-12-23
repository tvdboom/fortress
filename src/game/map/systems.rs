use super::components::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
    // Insert camera
    commands.spawn(Camera2d);

    // Insert background image
    commands.spawn((
        Sprite {
            image: asset_server.load("map/map.png"),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        },
        Map,
    ));

    // Insert wall
    commands.spawn((
        Sprite {
            image: asset_server.load("map/wall.png"),
            custom_size: Some(Vec2::new(window.width(), window.height() * 0.1)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., -window.height() / 3., 1.0)),
        Wall,
    ));
}
