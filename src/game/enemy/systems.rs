use super::components::*;
use crate::game::enemy::components::*;
use crate::game::map::components::*;
use crate::game::resources::{EnemyStatus, WaveStats};
use bevy::color::palettes::basic::{BLACK, LIME};
use bevy::color::Color;
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    window: Single<&Window>,
    asset_server: Res<AssetServer>,
    mut wave_stats: ResMut<WaveStats>,
) {
    let enemy = Walker::default();

    let mut rng = thread_rng();
    let window_half = window.width() / 2.;
    let random_number = rng.gen_range(-window_half + enemy.size..=window_half - enemy.size);

    wave_stats
        .enemies
        .entry(enemy.name.clone())
        .and_modify(|e| e.alive += 1)
        .or_insert_with(|| EnemyStatus {
            alive: 1,
            killed: 0,
        });

    commands
        .spawn((
            Sprite {
                image: asset_server.load(format!("enemy/{}.png", enemy.name.to_lowercase())),
                custom_size: Some(Vec2::new(enemy.size, enemy.size)),
                ..default()
            },
            Transform::from_xyz(random_number, window.height() / 2., 2.0),
            enemy.clone(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Sprite {
                        color: Color::from(BLACK),
                        custom_size: Some(Vec2::new(enemy.size * 0.8, enemy.size * 0.1)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, enemy.size / 2.0 - 5.0, 1.5),
                    LifeBarWrapper,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            color: Color::from(LIME),
                            custom_size: Some(Vec2::new(
                                enemy.size * 0.8 - 5.0,
                                enemy.size * 0.1 - 2.0,
                            )),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 1.6),
                        LifeBar,
                    ));
                });
        });
}

pub fn move_enemies(
    mut enemy_q: Query<(&mut Transform, &Walker)>,
    wall_q: Query<(&Transform, &Sprite), (With<Wall>, Without<Walker>)>,
    window: Single<&Window>,
    time: Res<Time>,
) {
    let (t, wall) = wall_q.iter().next().unwrap();
    let wall_y = t.translation.y + wall.custom_size.unwrap().y / 2.0;

    for (mut transform, enemy) in enemy_q.iter_mut() {
        let new_pos =
            transform.translation.y - window.height() / 100. * enemy.speed * time.delta_secs();

        if new_pos < wall_y + 5.0 {
            transform.translation.y = wall_y + 5.0;
        } else {
            transform.translation.y = new_pos;
        }
    }
}
