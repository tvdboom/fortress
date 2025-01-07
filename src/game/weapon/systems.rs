use crate::constants::*;
use crate::game::enemy::components::Enemy;
use crate::game::map::components::Map;
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::{Bullet, Fence, Wall, Weapon, WeaponManager};
use crate::utils::collision;
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub fn spawn_weapons(
    mut commands: Commands,
    player: Res<Player>,
    weapons: Res<WeaponManager>,
    asset_server: Res<AssetServer>,
) {
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

    // Spawn spots
    let positions = player
        .weapons
        .spots
        .iter()
        .enumerate()
        .map(|(i, _)| (i + 1) as f32 * MAP_SIZE.x / (player.weapons.spots.len() + 1) as f32)
        .collect::<Vec<f32>>();

    for (weapon, pos) in player.weapons.spots.iter().zip(positions) {
        if let Some(w) = weapon {
            let mut w = weapons.get(&w);
            w.update(&player);

            commands.spawn((
                Sprite {
                    image: asset_server.load(&w.image),
                    custom_size: Some(w.size),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(
                        -SIZE.x * 0.5 + pos,
                        -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 0.5,
                        2.0,
                    ),
                    rotation: Quat::from_rotation_z(PI * 0.5),
                    ..default()
                },
                w,
            ));
        }
    }

    // Spawn landmines
    let mut positions = vec![];
    let size = weapons.landmine.size;
    while positions.len() < player.weapons.landmines as usize {
        let x = thread_rng()
            .gen_range(-SIZE.x * 0.5 + size.x..=SIZE.x * 0.5 - WEAPONS_PANEL_SIZE.x - size.x);
        let y = thread_rng().gen_range(
            -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 1.6
                ..=SIZE.y * 0.5 - MENU_PANEL_SIZE.y - size.y,
        );
        let pos = Vec2::new(x, y);

        // Check landmines spawn at a minimum distance from each other
        if positions.iter().all(|&v| pos.distance(v) >= 2. * size.x) {
            positions.push(pos);

            commands.spawn((
                Sprite {
                    image: asset_server.load(&weapons.landmine.image),
                    custom_size: Some(weapons.landmine.size),
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 1.5),
                weapons.landmine.clone(),
            ));
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut weapon_q: Query<(&mut Transform, &mut Weapon), Without<Enemy>>,
    enemy_q: Query<(&Transform, &Enemy)>,
    map_q: Query<&Sprite, With<Map>>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let map_height = map_q.get_single().unwrap().custom_size.unwrap().y;

    for (mut transform, mut weapon) in weapon_q.iter_mut() {
        // Select an enemy in range
        if let Some(enemy_t) = weapon.select_target(&transform, &enemy_q, map_height) {
            if player.resources >= weapon.fire_cost {
                // Compute the angle to the selected enemy
                let d = enemy_t.translation - transform.translation;
                let angle = d.y.atan2(d.x);

                // Rotate the weapon towards the selected enemy
                if weapon.is_aiming(&angle, &transform) {
                    // Check if the weapon can fire (fire timer is finished)
                    if weapon.can_fire(&time, &game_settings) {
                        let mut bullet = weapon.bullet.clone();
                        bullet.angle = angle;

                        commands.spawn((
                            Sprite {
                                image: asset_server.load(&bullet.image),
                                custom_size: Some(bullet.size),
                                ..default()
                            },
                            Transform {
                                translation: Vec3::new(
                                    transform.translation.x + weapon.size.x * 0.5 * angle.cos(),
                                    transform.translation.y + weapon.size.y * 0.5 * angle.sin(),
                                    3.0,
                                ),
                                rotation: Quat::from_rotation_z(bullet.angle),
                                ..default()
                            },
                            bullet,
                        ));

                        night_stats.resources.bullets += weapon.fire_cost.bullets;
                        night_stats.resources.gasoline += weapon.fire_cost.gasoline;
                        player.resources.bullets -= weapon.fire_cost.bullets;
                        player.resources.gasoline -= weapon.fire_cost.gasoline;
                    }
                }

                // Rotate towards the enemy
                transform.rotation = transform.rotation.slerp(
                    Quat::from_rotation_z(angle),
                    weapon.rotation_speed * game_settings.speed * time.delta_secs(),
                );

                continue;
            }
        }

        // If the weapon couldn't shoot, return to the default position
        transform.rotation = transform.rotation.slerp(
            Quat::from_rotation_z(PI * 0.5),
            weapon.rotation_speed * game_settings.speed * time.delta_secs(),
        );
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
                &enemy.dim,
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
