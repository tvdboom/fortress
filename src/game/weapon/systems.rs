use crate::constants::*;
use crate::game::enemy::components::Enemy;
use crate::game::map::components::Map;
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::{Bullet, Fence, Wall, Weapon};
use crate::utils::collision;
use bevy::prelude::*;

pub fn spawn_weapons(mut commands: Commands, player: Res<Player>, asset_server: Res<AssetServer>) {
    if player.fence.max_health > 0. {
        commands.spawn((
            Sprite {
                image: asset_server.load("map/fence.png"),
                custom_size: Some(Vec2::new(WALL_SIZE.x, WALL_SIZE.y * 0.3)),
                ..default()
            },
            Transform::from_xyz(
                -WEAPONS_PANEL_SIZE.x * 0.5,
                -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 1.4,
                0.1,
            ),
            Fence,
        ));
    }

    commands.spawn((
        Sprite {
            image: asset_server.load("map/wall.png"),
            custom_size: Some(WALL_SIZE),
            ..default()
        },
        Transform::from_xyz(
            -WEAPONS_PANEL_SIZE.x * 0.5,
            -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 0.5,
            0.1,
        ),
        Wall,
    ));

    let positions = player
        .weapons
        .spots
        .iter()
        .enumerate()
        .map(|(i, _)| (i + 1) as f32 * MAP_SIZE.x / (player.weapons.spots.len() + 1) as f32)
        .collect::<Vec<f32>>();

    for (weapon, pos) in player.weapons.spots.iter().zip(positions) {
        if let Some(w) = weapon {
            let params = player.weapons.settings.get(w);

            commands.spawn((
                Sprite {
                    image: asset_server.load(&params.image),
                    custom_size: Some(params.size),
                    ..default()
                },
                Transform::from_xyz(
                    -SIZE.x * 0.5 + pos,
                    -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 0.5,
                    2.0,
                ),
                w.clone(),
            ));
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut weapon_q: Query<(&Transform, &mut Weapon)>,
    enemy_q: Query<&Transform, With<Enemy>>,
    map_q: Query<&Sprite, With<Map>>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let map_height = map_q.get_single().unwrap().custom_size.unwrap().y;

    for (transform, mut weapon) in weapon_q.iter_mut() {
        let params = player.weapons.settings.get(weapon.as_ref()).clone();

        if weapon.can_fire(&time)
            && player.resources.bullets > params.fire_cost.bullets
            && player.resources.gasoline > params.fire_cost.gasoline
        {
            // Find the nearest enemy in range
            if let Some((nearest_enemy, distance)) = enemy_q
                .iter()
                .map(|enemy| {
                    let distance = transform.translation.distance(enemy.translation);
                    (enemy, distance)
                })
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            {
                // The weapon's fire range is a percentage of the map's height
                if distance <= map_height / 100. * params.bullet.max_distance {
                    let mut bullet = params.bullet.clone();

                    // Compute the angle to the nearest enemy
                    let d = nearest_enemy.translation - transform.translation;
                    bullet.angle = d.y.atan2(d.x);

                    commands.spawn((
                        Sprite {
                            image: asset_server.load(&bullet.image),
                            custom_size: Some(bullet.size),
                            ..default()
                        },
                        Transform {
                            translation: Vec3::new(
                                transform.translation.x,
                                transform.translation.y + 10.,
                                3.0,
                            ),
                            rotation: Quat::from_rotation_z(bullet.angle),
                            ..default()
                        },
                        bullet,
                    ));

                    night_stats.resources.bullets += params.fire_cost.bullets;
                    night_stats.resources.gasoline += params.fire_cost.gasoline;
                    player.resources.bullets -= params.fire_cost.bullets;
                    player.resources.gasoline -= params.fire_cost.gasoline;
                }
            }
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut bullet_q: Query<(&mut Transform, Entity, &mut Bullet)>,
    mut enemy_q: Query<(&Transform, Entity, &mut Enemy), Without<Bullet>>,
    map_q: Query<&Sprite, With<Map>>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut night_stats: ResMut<NightStats>,
) {
    let map_height = map_q.get_single().unwrap().custom_size.unwrap().y;

    for (mut transform, entity, mut bullet) in bullet_q.iter_mut() {
        let dx = map_height / 100.
            * bullet.speed
            * bullet.angle.cos()
            * settings.speed
            * time.delta_secs();
        transform.translation.x += dx;

        let dy = map_height / 100.
            * bullet.speed
            * bullet.angle.sin()
            * settings.speed
            * time.delta_secs();
        transform.translation.y += dy;

        // Pythagoras to get distance traveled
        bullet.distance += (dx.powi(2) + dy.powi(2)).sqrt();

        // If the bullet collided with an enemy -> resolve and despawn
        for (transform_enemy, enemy_entity, mut enemy) in enemy_q.iter_mut() {
            if collision(
                &transform.translation,
                &bullet.size,
                &transform_enemy.translation,
                &enemy.size,
            ) {
                commands.entity(entity).despawn();

                let damage = bullet.damage - enemy.armor;
                if enemy.health <= damage {
                    commands.entity(enemy_entity).despawn_recursive();

                    night_stats
                        .enemies
                        .entry(enemy.name)
                        .and_modify(|status| status.killed += 1);
                } else {
                    enemy.health -= damage;
                }
            }
        }

        // If the bullet traveled more than max distance or left window boundaries -> despawn
        if bullet.distance >= map_height / 100. * bullet.max_distance
            || transform.translation.x < -SIZE.x * 0.5
            || transform.translation.x > SIZE.x * 0.5
            || transform.translation.y > SIZE.y * 0.5
            || transform.translation.y < -SIZE.y * 0.5
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_fence_resources(
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    if player.fence.enabled {
        let cost = player.fence.cost.gasoline * game_settings.speed * time.delta_secs();
        if player.resources.gasoline >= cost {
            player.resources.gasoline -= cost;
        } else {
            player.fence.enabled = false;
        }
    }
}
