use super::components::*;
use crate::game::components::*;
use crate::game::enemy::components::{Enemy, EnemyHealth};
use crate::game::weapon::components::Weapon;
use crate::resources::Player;
use crate::{HEIGHT, WIDTH};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, player: Res<Player>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("map/grass.png"),
            custom_size: Some(Vec2::new(WIDTH, HEIGHT * 0.8)),
            ..default()
        },
        Transform::from_xyz(0., HEIGHT * 0.1, 0.0),
        Map,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("map/wall.png"),
            custom_size: Some(Vec2::new(WIDTH, HEIGHT * 0.1)),
            ..default()
        },
        Transform::from_xyz(0., -HEIGHT * 0.35, 1.0),
        Wall,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("map/sand.png"),
            custom_size: Some(Vec2::new(WIDTH, HEIGHT * 0.1)),
            ..default()
        },
        Transform::from_xyz(0., -HEIGHT * 0.45, 1.0),
        Sand,
    ));

    // Spawn hidden pause banner
    commands
        .spawn((
            Sprite {
                color: Color::srgba(255., 255., 255., 0.1),
                custom_size: Some(Vec2::new(WIDTH * 0.1, HEIGHT * 0.1)),
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
                Transform::from_xyz(0., 0., 3.1),
                PauseText,
            ));
        });

    // Spawn resources
    commands
        .spawn((
            Sprite {
                color: Color::srgba(255., 255., 255., 0.2),
                custom_size: Some(Vec2::new(WIDTH * 0.49, HEIGHT * 0.085)),
                ..default()
            },
            Transform::from_xyz(-WIDTH * 0.25, -HEIGHT * 0.45, 1.1),
            ResourcesWrapper,
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load("map/health.png"),
                    custom_size: Some(Vec2::new(WIDTH * 0.03, HEIGHT * 0.04)),
                    ..default()
                },
                Transform::from_xyz(-WIDTH * 0.24, 0., 1.2),
            ));
            parent.spawn((
                Text2d::new(player.wall.health.to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_xyz(-WIDTH * 0.18, 0., 1.2),
                HealthText,
            ));
            parent.spawn((
                Sprite {
                    image: asset_server.load("map/bullets.png"),
                    custom_size: Some(Vec2::new(WIDTH * 0.03, HEIGHT * 0.04)),
                    ..default()
                },
                Transform::from_xyz(-WIDTH * 0.10, 0., 1.2),
            ));
            parent.spawn((
                Text2d::new(player.resources.bullets.to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_xyz(-WIDTH * 0.04, 0., 1.2),
                BulletsText,
            ));
            parent.spawn((
                Sprite {
                    image: asset_server.load("map/gasoline.png"),
                    custom_size: Some(Vec2::new(WIDTH * 0.03, HEIGHT * 0.04)),
                    ..default()
                },
                Transform::from_xyz(-WIDTH * 0.08, 0., 1.2),
            ));
            parent.spawn((
                Text2d::new(player.resources.gasoline.to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_xyz(-WIDTH * 0.04, 0., 1.2),
                GasolineText,
            ));
            parent.spawn((
                Sprite {
                    image: asset_server.load("map/materials.png"),
                    custom_size: Some(Vec2::new(WIDTH * 0.03, HEIGHT * 0.04)),
                    ..default()
                },
                Transform::from_xyz(WIDTH * 0.02, 0., 1.2),
            ));
            parent.spawn((
                Text2d::new(player.resources.materials.to_string()),
                TextColor(Color::from(WHITE)),
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_xyz(WIDTH * 0.06, 0., 1.2),
                MaterialsText,
            ));
        });

    // Spawn sentry-guns
    let weapon = Weapon::sentry_gun();

    let mut pos = -WIDTH * 0.5;
    for _ in 0..player.weapons.sentry_gun {
        pos += WIDTH / (player.weapons.sentry_gun + 1) as f32;

        commands.spawn((
            Sprite {
                image: asset_server.load(&weapon.image),
                custom_size: Some(Vec2::new(weapon.size.0, weapon.size.1)),
                ..default()
            },
            Transform::from_xyz(pos, -HEIGHT * 0.35, 2.0),
            weapon.clone(),
        ));
    }
}

pub fn map_update(
    mut text_q: Query<(
        &mut Text2d,
        Option<&HealthText>,
        Option<&BulletsText>,
        Option<&GasolineText>,
        Option<&MaterialsText>,
    )>,
    mut enemy_q: Query<(&mut Sprite, &Enemy), With<EnemyHealth>>,
    player: Res<Player>,
) {
    // Update health and resources
    for (mut text, health, bullets, gasoline, materials) in text_q.iter_mut() {
        if health.is_some() {
            text.0 = player.wall.health.to_string();
        } else if bullets.is_some() {
            text.0 = player.resources.bullets.to_string();
        } else if gasoline.is_some() {
            text.0 = player.resources.gasoline.to_string();
        } else if materials.is_some() {
            text.0 = player.resources.materials.to_string();
        }
    }

    // Update enemy health bars
    for (mut sprite, enemy) in enemy_q.iter_mut() {
        println!("{:?}, {:?}", sprite, enemy);
        sprite.custom_size.unwrap().x = sprite.custom_size.unwrap().x * (enemy.health / enemy.max_health) as f32
    }
}
