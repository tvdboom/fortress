use super::components::*;
use crate::game::components::*;
use crate::game::map::components::*;
use crate::game::map::constants::{MAP_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::resources::{EnemyStatus, Player, WaveStats};
use crate::game::systems::pause_game;
use crate::game::GameState;
use bevy::color::{
    palettes::basic::{BLACK, LIME},
    Color,
};
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    mut wave_stats: ResMut<WaveStats>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = thread_rng();
    let enemy = match rng.gen_range(0..1000) * wave_stats.day {
        800..950 => Enemy::walker(),
        950..990 => Enemy::runner(),
        990..1000 => Enemy::dragon(),
        _ => return,
    };

    let x = rng.gen_range(
        (-SIZE.x + enemy.size.x) * 0.5..=(SIZE.x - enemy.size.x) * 0.5 - WEAPONS_PANEL_SIZE.x,
    );

    commands
        .spawn((
            Sprite {
                image: asset_server.load(&enemy.image),
                custom_size: Some(enemy.size),
                ..default()
            },
            Transform::from_xyz(x, SIZE.y * 0.5, 2.0),
            enemy.clone(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Sprite {
                        color: Color::from(BLACK),
                        custom_size: Some(Vec2::new(enemy.size.x * 0.8, enemy.size.y * 0.1)),
                        ..default()
                    },
                    Transform::from_xyz(0., enemy.size.y * 0.5 - 5.0, 1.5),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            color: Color::from(LIME),
                            custom_size: Some(Vec2::new(
                                enemy.size.x * 0.8 - 2.0,
                                enemy.size.y * 0.1 - 2.0,
                            )),
                            ..default()
                        },
                        Transform::from_xyz(0., 0., 1.6),
                        EnemyHealth,
                    ));
                });
        });

    wave_stats
        .enemies
        .entry(enemy.name.clone())
        .and_modify(|status| status.spawned += 1)
        .or_insert_with(|| EnemyStatus {
            spawned: 1,
            killed: 0,
        });
}

pub fn move_enemies(
    mut enemy_q: Query<(&mut Transform, &Enemy)>,
    wall_q: Query<(&Transform, &Sprite), (With<Wall>, Without<Enemy>)>,
    vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut player: ResMut<Player>,
    next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let (t, wall) = wall_q.iter().next().unwrap();
    let wall_y = t.translation.y + wall.custom_size.unwrap().y * 0.5;

    for (mut transform, enemy) in enemy_q.iter_mut() {
        let new_pos = transform.translation.y - MAP_SIZE.y / 100. * enemy.speed * time.delta_secs();

        if new_pos < wall_y + 5. {
            transform.translation.y = wall_y + 5.;

            if player.wall.health > enemy.damage {
                player.wall.health -= enemy.damage;
            } else {
                pause_game(vis_q, next_state);
                todo!();
            }
        } else {
            transform.translation.y = new_pos;
        }
    }
}

pub fn update_enemy_health_bars(
    enemy_q: Query<(&Enemy, Entity)>,
    children_q: Query<&Children>,
    mut health_q: Query<(&mut Transform, &mut Sprite), With<EnemyHealth>>,
) {
    for (enemy, entity) in enemy_q.iter() {
        if enemy.health < enemy.max_health {
            for child in children_q.iter_descendants(entity) {
                if let Ok((mut transform, mut sprite)) = health_q.get_mut(child) {
                    if let Some(size) = sprite.custom_size.as_mut() {
                        let full_size = enemy.size.x * 0.8 - 2.0;
                        size.x = full_size * enemy.health as f32 / enemy.max_health as f32;
                        transform.translation.x = (size.x - full_size) * 0.5;
                    }
                }
            }
        }
    }
}
