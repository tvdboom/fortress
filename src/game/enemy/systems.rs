use super::components::*;
use crate::constants::{RESOURCES_PANEL_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::assets::WorldAssets;
use crate::game::map::components::AnimationComponent;
use crate::game::resources::{EnemyStatus, GameSettings, NightStats, Player};
use crate::game::weapon::components::{Fence, Landmine, Wall};
use crate::game::AppState;
use crate::utils::{collision, scale_duration};
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
                    Transform::from_xyz(x, SIZE.y * 0.5, 2.0),
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
                            Transform::from_xyz(0., enemy.dim.y * 0.5 - 5.0, 1.5),
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
                                Transform::from_xyz(0., 0., 1.6),
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
    landmine_q: Query<(Entity, &Transform, &Landmine), (With<Landmine>, Without<Enemy>)>,
    fence_q: Query<(Entity, &Transform, &Sprite), (With<Fence>, Without<Enemy>)>,
    wall_q: Query<(Entity, &Transform, &Sprite), (With<Wall>, Without<Enemy>)>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut night_stats: ResMut<NightStats>,
    assets: Local<WorldAssets>,
) {
    for (enemy_entity, mut transform, mut enemy) in enemy_q.iter_mut() {
        let new_pos =
            transform.translation.y - enemy.speed * game_settings.speed * time.delta_secs();

        if !enemy.can_fly {
            if let Some((entity, t, fence)) = fence_q.iter().next() {
                let fence_y = t.translation.y + fence.custom_size.unwrap().y * 0.5;

                if new_pos < fence_y + 5. {
                    transform.translation.y = fence_y + 5.;

                    if player.fence.health > enemy.damage {
                        player.fence.health -=
                            enemy.damage * game_settings.speed * time.delta_secs();
                        if player.fence.enabled {
                            let damage =
                                player.fence.damage * game_settings.speed * time.delta_secs();

                            if enemy.health > damage {
                                enemy.health -= damage;
                            } else {
                                commands.entity(enemy_entity).try_despawn_recursive();

                                night_stats
                                    .enemies
                                    .entry(enemy.name)
                                    .and_modify(|status| status.killed += 1);
                            }
                        }
                    } else {
                        player.fence.health = 0.;
                        commands.entity(entity).try_despawn();
                    }

                    continue;
                }
            } else if let Some((entity, t, wall)) = wall_q.iter().next() {
                let wall_y = t.translation.y + wall.custom_size.unwrap().y * 0.5;

                if new_pos < wall_y + 5. {
                    transform.translation.y = wall_y + 5.;

                    if player.wall.health > enemy.damage {
                        player.wall.health -=
                            enemy.damage * game_settings.speed * time.delta_secs();
                    } else {
                        player.wall.health = 0.;
                        commands.entity(entity).try_despawn();
                    }

                    continue;
                }
            }
        }

        if new_pos < -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y - enemy.dim.y * 0.5 {
            if player.survivors > enemy.damage as u32 {
                player.survivors -= enemy.damage as u32;
                commands.entity(enemy_entity).try_despawn_recursive();
            } else {
                player.survivors = 0;
                next_state.set(AppState::GameOver);
            }
        } else {
            transform.translation.y = new_pos;

            // Check collision with landmines
            if !enemy.can_fly && enemy.size >= player.weapons.settings.landmine_sensibility {
                for (landmine_entity, landmine_t, landmine) in landmine_q.iter() {
                    if collision(
                        &transform.translation,
                        &enemy.dim,
                        &landmine_t.translation,
                        &landmine.dim,
                    ) {
                        player.weapons.landmines -= 1;
                        commands.entity(landmine_entity).try_despawn();

                        let atlas = assets.get_atlas(&landmine.atlas);
                        commands.spawn((
                            Sprite {
                                image: atlas.image,
                                texture_atlas: Some(atlas.texture),
                                custom_size: Some(Vec2::splat(landmine.explosion.radius)),
                                ..default()
                            },
                            Transform::from_translation(transform.translation),
                            AnimationComponent {
                                timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                                last_index: atlas.last_index,
                                explosion: Some(landmine.explosion.clone()),
                            },
                        ));
                    }
                }
            }
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
                        let full_size = enemy.dim.x * 0.8 - 2.0;
                        size.x = full_size * enemy.health / enemy.max_health;
                        transform.translation.x = (size.x - full_size) * 0.5;
                    }
                }
            }
        }
    }
}
