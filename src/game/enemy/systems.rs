use super::components::*;
use crate::constants::{EnemyQ, SpriteQ, ENEMY_Z, RESOURCES_PANEL_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::resources::{EnemyStatus, GameSettings, NightStats, Player};
use crate::game::weapon::components::{Fence, Wall};
use crate::game::weapon::utils::get_structure_top;
use crate::game::AppState;
use crate::utils::scale_duration;
use bevy::color::{
    palettes::basic::{BLACK, LIME},
    Color,
};
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    enemy_q: Query<&Enemy>,
    enemies: Res<EnemyManager>,
    mut night_stats: ResMut<NightStats>,
    mut next_state: ResMut<NextState<AppState>>,
    game_settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // Stop spawning enemies when the night timer has finished
    if night_stats.timer.finished() {
        if enemy_q.iter().count() == 0 {
            next_state.set(AppState::EndNight);
        }
        return;
    }

    night_stats
        .spawn_timer
        .tick(scale_duration(time.delta(), game_settings.speed));

    if night_stats.spawn_timer.just_finished() {
        if let Some(enemy) =
            enemies.choose_enemy(night_stats.day, night_stats.timer.elapsed().as_secs_f32())
        {
            let x = thread_rng().gen_range(
                (-SIZE.x + enemy.dim.x) * 0.5..=(SIZE.x - enemy.dim.x) * 0.5 - WEAPONS_PANEL_SIZE.x,
            );

            commands
                .spawn((
                    Sprite {
                        image: asset_server.load(enemy.image),
                        custom_size: Some(enemy.dim),
                        ..default()
                    },
                    Transform::from_xyz(x, SIZE.y * 0.5, ENEMY_Z),
                    enemy.clone(),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Sprite {
                                color: Color::from(BLACK),
                                custom_size: Some(Vec2::new(enemy.dim.x * 0.8, enemy.dim.y * 0.1)),
                                ..default()
                            },
                            Transform::from_xyz(0., enemy.dim.y * 0.5 - 5.0, ENEMY_Z + 0.1),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Sprite {
                                    color: Color::from(LIME),
                                    custom_size: Some(Vec2::new(
                                        enemy.dim.x * 0.78,
                                        enemy.dim.y * 0.08,
                                    )),
                                    ..default()
                                },
                                Transform::from_xyz(0., 0., ENEMY_Z + 0.2),
                                EnemyHealth,
                            ));
                        });
                });

            night_stats
                .enemies
                .entry(enemy.name)
                .and_modify(|status| status.spawned += 1)
                .or_insert_with(|| EnemyStatus {
                    spawned: 1,
                    killed: 0,
                });
        }
    }
}

pub fn move_enemies(
    mut commands: Commands,
    mut enemy_q: Query<(Entity, &mut Transform, &mut Enemy)>,
    fence_q: Query<SpriteQ, (With<Fence>, Without<Enemy>)>,
    wall_q: Query<SpriteQ, (With<Wall>, Without<Enemy>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    for (enemy_e, mut enemy_t, mut enemy) in enemy_q.iter_mut() {
        let new_pos = enemy_t.translation.y - enemy.speed * game_settings.speed * time.delta_secs();

        if !enemy.can_fly {
            if let Some(fence_y) = get_structure_top(fence_q.get_single()) {
                if new_pos < fence_y {
                    enemy_t.translation.y = fence_y;

                    if player.fence.health > enemy.damage {
                        player.fence.health -=
                            enemy.damage * game_settings.speed * time.delta_secs();
                        if player.fence.enabled {
                            let damage =
                                player.fence.damage * game_settings.speed * time.delta_secs();

                            if enemy.health > damage {
                                enemy.health -= damage;
                            } else {
                                commands.entity(enemy_e).try_despawn_recursive();

                                night_stats
                                    .enemies
                                    .entry(enemy.name)
                                    .and_modify(|status| status.killed += 1);
                            }
                        }
                    } else {
                        player.fence.health = 0.;
                        commands
                            .entity(fence_q.get_single().unwrap().0)
                            .try_despawn();
                    }

                    continue;
                }
            } else if let Some(wall_y) = get_structure_top(wall_q.get_single()) {
                if new_pos < wall_y {
                    enemy_t.translation.y = wall_y;

                    if player.wall.health > enemy.damage {
                        player.wall.health -=
                            enemy.damage * game_settings.speed * time.delta_secs();
                    } else {
                        player.wall.health = 0.;
                        commands
                            .entity(wall_q.get_single().unwrap().0)
                            .try_despawn();
                    }

                    continue;
                }
            }
        }

        if new_pos < -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y - enemy.dim.y * 0.5 {
            if player.survivors > enemy.damage as u32 {
                player.survivors -= enemy.damage as u32;
                commands.entity(enemy_e).try_despawn_recursive();
            } else {
                player.survivors = 0;
                next_state.set(AppState::GameOver);
            }
        } else {
            enemy_t.translation.y = new_pos;
        }
    }
}

pub fn update_enemy_health_bars(
    enemy_q: Query<EnemyQ>,
    children_q: Query<&Children>,
    mut health_q: Query<(&mut Transform, &mut Sprite), With<EnemyHealth>>,
) {
    for (entity, _, enemy) in enemy_q.iter() {
        if enemy.health < enemy.max_health {
            for child in children_q.iter_descendants(entity) {
                if let Ok((mut sprite_t, mut sprite)) = health_q.get_mut(child) {
                    if let Some(size) = sprite.custom_size.as_mut() {
                        let full_size = enemy.dim.x * 0.8 - 2.0;
                        size.x = full_size * enemy.health / enemy.max_health;
                        sprite_t.translation.x = (size.x - full_size) * 0.5;
                    }
                }
            }
        }
    }
}
