use super::components::*;
use crate::game::enemy::components::*;
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    window: Single<&Window>,
    asset_server: Res<AssetServer>,
) {
    let enemy = Walker::default();

    let mut rng = thread_rng();
    let window_half = window.width() / 2.;
    let random_number = rng.gen_range(-window_half..=window_half);

    commands.spawn((
        Sprite {
            image: asset_server.load("enemy/walker.png"),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(random_number, window.height() / 2., 2.0)),
        enemy,
    ));
}
