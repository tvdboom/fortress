use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::Enemy;
use crate::game::map::components::AnimationComponent;
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::*;
use crate::utils::collision;
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
                    custom_size: Some(w.dim),
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
    let size = weapons.landmine.dim;
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
                    custom_size: Some(weapons.landmine.dim),
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
    enemy_q: Query<(&Transform, Entity, &Enemy)>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    for (mut transform, mut weapon) in weapon_q.iter_mut() {
        // Select a target in range
        if let Some((t, e)) = weapon.get_lock(&transform, &enemy_q, &player) {
            if player.resources >= weapon.fire_cost {
                // let mut bullet = weapon.bullet.clone();

                // Compute the angle to the selected enemy
                let d = t.translation - transform.translation;

                // let relative_velocity_factor = (d.y / bullet.speed) - (enemy_velocity / bullet_velocity);
                // let angle = relative_velocity_factor.atan2(d.x);

                let angle = d.y.atan2(d.x);

                // Rotate the weapon towards the selected target
                if weapon.is_aiming(&angle, &transform) {
                    // Check if the weapon can fire (fire timer is finished)
                    if weapon.can_fire(&time, &game_settings) {
                        // Release target lock
                        weapon.lock = None;

                        let mut bullet = Bullet {
                            angle,
                            ..weapon.bullet.clone()
                        };

                        if matches!(weapon.fire_strategy, FireStrategy::Density { .. }) {
                            bullet.max_distance = d.length() - bullet.dim.length();
                        }

                        commands.spawn((
                            assets.get_atlas("flash1"),
                            Transform {
                                translation: Vec3::new(
                                    transform.translation.x + weapon.dim.x * angle.cos() + 3.,
                                    transform.translation.y + weapon.dim.y * angle.sin(),
                                    4.0,
                                ),
                                rotation: Quat::from_rotation_z(bullet.angle),
                                ..default()
                            },
                            AnimationComponent {
                                timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                                last_index: 30,
                            },
                        ));

                        commands.spawn((
                            Sprite {
                                image: asset_server.load(&bullet.image),
                                custom_size: Some(bullet.dim),
                                ..default()
                            },
                            Transform {
                                translation: Vec3::new(
                                    transform.translation.x + weapon.dim.x * 0.5 * angle.cos(),
                                    transform.translation.y + weapon.dim.y * 0.5 * angle.sin(),
                                    3.0,
                                ),
                                rotation: Quat::from_rotation_z(bullet.angle),
                                ..default()
                            },
                            bullet,
                        ));

                        night_stats.resources += &weapon.fire_cost;
                        player.resources -= &weapon.fire_cost;
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
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut night_stats: ResMut<NightStats>,
    assets: Local<WorldAssets>,
) {
    for (mut transform, entity, mut bullet) in bullet_q.iter_mut() {
        let dx = bullet.speed * bullet.angle.cos() * settings.speed * time.delta_secs();
        transform.translation.x += dx;

        let dy = bullet.speed * bullet.angle.sin() * settings.speed * time.delta_secs();
        transform.translation.y += dy;

        // Pythagoras to get distance traveled
        bullet.distance += (dx.powi(2) + dy.powi(2)).sqrt();

        match bullet.detonation {
            Detonation::SingleTarget => {
                for (transform_enemy, enemy_entity, mut enemy) in enemy_q.iter_mut() {
                    // If the bullet can hit grounded/airborne enemies
                    // and the bullet collided with an enemy -> despawn and resolve
                    if ((bullet.damage.ground > 0. && !enemy.can_fly)
                        || (bullet.damage.air > 0. && enemy.can_fly))
                        && collision(
                            &transform.translation,
                            &bullet.dim,
                            &transform_enemy.translation,
                            &enemy.dim,
                        )
                    {
                        commands.entity(entity).despawn();
                        resolve_enemy_impact(
                            &mut commands,
                            &mut bullet,
                            enemy_entity,
                            &mut enemy,
                            &mut night_stats,
                        );
                        break;
                    }
                }
            }
            Detonation::Explosion(r) if bullet.distance >= bullet.max_distance => {
                commands.entity(entity).despawn();

                commands.spawn((
                    assets.get_atlas("explosion1"),
                    Transform::from_translation(transform.translation),
                    AnimationComponent {
                        timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                        last_index: 25,
                    },
                ));

                // Resolve the impact on all enemies in radius
                enemy_q
                    .iter_mut()
                    .filter(|(&t, _, _)| t.translation.distance(transform.translation) <= r as f32)
                    .for_each(|(_, enemy_entity, mut enemy)| {
                        resolve_enemy_impact(
                            &mut commands,
                            &mut bullet,
                            enemy_entity,
                            &mut enemy,
                            &mut night_stats,
                        )
                    });

                break;
            }
            _ => (),
        }

        // If the bullet traveled more than max distance or left window boundaries -> despawn
        if bullet.distance >= bullet.max_distance
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

fn resolve_enemy_impact(
    commands: &mut Commands,
    bullet: &mut Bullet,
    enemy_entity: Entity,
    enemy: &mut Enemy,
    night_stats: &mut NightStats,
) {
    let damage = bullet.damage.calculate(enemy);
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
