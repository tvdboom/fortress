use super::components::*;
use crate::constants::{MAP_SIZE, RESOURCES_PANEL_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::map::components::*;
use crate::game::resources::{EnemyStatus, GameSettings, NightStats, Player};
use crate::game::AppState;
use bevy::color::{
    palettes::basic::{BLACK, LIME},
    Color,
};
use bevy::prelude::*;
use rand::prelude::*;
use crate::game::weapon::components::WeaponSettings;

pub fn spawn_enemies(
    mut commands: Commands,
    enemy_q: Query<&Enemy>,
    mut night_stats: ResMut<NightStats>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Stop spawning enemies when the night timer has finished
    if night_stats.timer.finished() {
        if enemy_q.iter().count() == 0 {
            next_state.set(AppState::EndNight);
        }
        return;
    }

    let mut rng = thread_rng();
    let enemy = match rng.gen_range(0..1000) * night_stats.day {
        800..900 => Enemy::walker(),
        900..950 => Enemy::runner(),
        950..970 => Enemy::ogre(),
        970..990 => Enemy::armored_ogre(),
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

    night_stats
        .enemies
        .entry(enemy.name.clone())
        .and_modify(|status| status.spawned += 1)
        .or_insert_with(|| EnemyStatus {
            spawned: 1,
            killed: 0,
        });
}

pub fn move_enemies(
    mut commands: Commands,
    mut enemy_q: Query<(Entity, &mut Transform, &Enemy)>,
    fence_q: Query<(Entity, &Transform, &Sprite), (With<Fence>, Without<Enemy>)>,
    wall_q: Query<(Entity, &Transform, &Sprite), (With<Wall>, Without<Enemy>)>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    weapon_settings: Res<WeaponSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    for (enemy_entity, mut transform, enemy) in enemy_q.iter_mut() {
        let new_pos = transform.translation.y
            - MAP_SIZE.y / 100. * enemy.speed * game_settings.speed * time.delta_secs();

        if player.fence.health > 0. {
            let (entity, t, fence) = fence_q.iter().next().unwrap();
            let fence_y = t.translation.y + fence.custom_size.unwrap().y * 0.5;

            if new_pos < fence_y + 5. {
                transform.translation.y = fence_y + 5.;

                if player.fence.health > enemy.damage {
                    player.fence.health -= enemy.damage * game_settings.speed * time.delta_secs();
                    if weapon_settings.fence {
                        enemy.health -=
                    }
                } else {
                    player.fence.health = 0.;
                    commands.entity(entity).despawn();
                }
            } else {
                transform.translation.y = new_pos;
            }
        } else if player.wall.health > 0. {
            let (entity, t, wall) = wall_q.iter().next().unwrap();
            let wall_y = t.translation.y + wall.custom_size.unwrap().y * 0.5;

            if new_pos < wall_y + 5. {
                transform.translation.y = wall_y + 5.;

                if player.wall.health > enemy.damage {
                    player.wall.health -= enemy.damage * game_settings.speed * time.delta_secs();
                } else {
                    player.wall.health = 0.;
                    commands.entity(entity).despawn();
                }
            } else {
                transform.translation.y = new_pos;
            }
        } else if new_pos < -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y - 10. {
            if player.survivors > enemy.damage as u32 {
                player.survivors -= (enemy.damage * game_settings.speed) as u32;
                commands.entity(enemy_entity).despawn_recursive();
            } else {
                player.survivors = 0;
                next_state.set(AppState::GameOver);
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
