use bevy::color::palettes::basic::{BLACK, GRAY, LIME};
use super::components::*;
use bevy::prelude::*;
use crate::game::components::*;
use crate::resources::Player;

pub fn setup(
    mut commands: Commands,
    player: Res<Player>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.,
        ..default()
    };

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("map/grass.png"),
            custom_size: Some(Vec2::new(window.width(), window.height() * 0.7)),
            ..default()
        },
        Transform::from_xyz(0., window.height() * 0.15, 0.0),
        Map,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("map/sand.png"),
            custom_size: Some(Vec2::new(window.width(), window.height() * 0.1)),
            ..default()
        },
        Transform::from_xyz(0., -window.height() * 0.45, 1.0),
        Map,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("map/wall.png"),
            custom_size: Some(Vec2::new(window.width(), window.height() * 0.1)),
            ..default()
        },
        Transform::from_xyz(0., -window.height() * 0.35, 1.0),
        Wall,
    ));

    commands.spawn((
        Sprite {
            color: Color::from(BLACK),
            custom_size: Some(Vec2::new(window.width() * 0.1, window.height() * 0.05)),
            ..default()
        },
        Transform::from_xyz(window.width() * 0.45, -window.height() * 0.45, 1.5),
        LifeBarWrapper,
    ))
    .with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::from(LIME),
                custom_size: Some(Vec2::new(window.width() * 0.1 - 5.0, window.height() * 0.05 - 5.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.1),
            LifeBar,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(player.wall.health.to_string()),
                text_font.clone(),
                TextColor(Color::from(GRAY)),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_xyz(0.0, 0.0, 1.1),
                LifeBarText,
            ));
        });
    });
}
