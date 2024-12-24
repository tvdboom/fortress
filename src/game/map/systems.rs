use super::components::*;
use crate::game::components::*;
use crate::resources::Player;
use bevy::color::palettes::basic::{BLACK, GRAY, LIME};
use bevy::prelude::*;

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
            custom_size: Some(Vec2::new(window.width(), window.height() * 0.8)),
            ..default()
        },
        Transform::from_xyz(0., window.height() * 0.1, 0.0),
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

    commands
        .spawn((
            Sprite {
                color: Color::from(BLACK),
                custom_size: Some(Vec2::new(window.width() * 0.1, window.height() * 0.05)),
                ..default()
            },
            Transform::from_xyz(window.width() * 0.42, -window.height() * 0.45, 1.5),
            LifeBarWrapper,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Sprite {
                        color: Color::from(LIME),
                        custom_size: Some(Vec2::new(
                            window.width() * 0.1 - 5.0,
                            window.height() * 0.05 - 5.0,
                        )),
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

pub fn text_update(
    mut wall_q: Query<&mut Text2d, With<LifeBarText>>,
    player: Res<Player>,
) {
    let mut span = wall_q.get_single_mut().unwrap();
    **span = player.wall.health.to_string();
}
