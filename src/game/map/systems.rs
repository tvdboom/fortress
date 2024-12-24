use super::components::*;
use crate::game::components::*;
use crate::game::weapon::components::Weapon;
use crate::resources::Player;
use bevy::color::palettes::basic::{BLACK, GRAY, LIME, WHITE};
use bevy::prelude::*;
use crate::game::enemy::components::{Enemy, EnemyHealth};

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
        Sand,
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
            WallHealthWrapper,
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
                    WallHealth,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2d::new(player.wall.health.to_string()),
                        text_font.clone(),
                        TextColor(Color::from(GRAY)),
                        TextLayout::new_with_justify(JustifyText::Center),
                        Transform::from_xyz(0.0, 0.0, 1.1),
                        WallHealthText,
                    ));
                });
        });

    // Spawn hidden pause banner
    commands
        .spawn((
            Sprite {
                color: Color::srgba(255., 255., 255., 0.1),
                custom_size: Some(Vec2::new(window.width() * 0.1, window.height() * 0.1)),
                ..default()
            },
            Transform::from_xyz(0., 0., 3.),
            Visibility::Hidden,
            PauseWrapper,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("Paused".to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_xyz(0.0, 0.0, 3.1),
                PauseText,
            ));
        });

    // Spawn sentry-gun
    let weapon = Weapon::sentry_gun();

    let mut pos = -window.width() * 0.5;
    for _ in 0..player.weapons.sentry_gun {
        pos += window.width() / (player.weapons.sentry_gun + 1) as f32;

        commands.spawn((
            Sprite {
                image: asset_server.load(&weapon.image),
                custom_size: Some(Vec2::new(weapon.size.0, weapon.size.1)),
                ..default()
            },
            Transform::from_xyz(pos, -window.height() * 0.35, 2.0),
            weapon.clone(),
        ));
    }
}

pub fn map_update(
    mut wall_q: Query<&mut Text2d, With<WallHealthText>>,
    mut enemy_q: Query<(&mut Sprite, &Enemy),  With<EnemyHealth>>,
    player: Res<Player>,
) {
    // Update wall health
    let mut span = wall_q.get_single_mut().unwrap();
    **span = player.wall.health.to_string();

    for (mut sprite, enemy) in enemy_q.iter_mut() {
        sprite.custom_size = Some(Vec2::new(
            sprite.custom_size.unwrap().x * (enemy.health / enemy.max_health) as f32,
            sprite.custom_size.unwrap().y,
        ))
    }
}
