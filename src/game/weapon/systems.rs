use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::Enemy;
use crate::game::map::components::{AnimationComponent, FogOfWar};
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
        let level = (player.fence.max_health as u32 / 100).min(3);
        commands.spawn((
            Sprite {
                image: asset_server.load(format!("map/fence{}.png", level)),
                custom_size: Some(Vec2::new(FENCE_SIZE.x, FENCE_SIZE.y * level as f32 / 3.)),
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
                    image: asset_server.load(w.image),
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

    // Spawn mines
    let mut positions = vec![];
    let size = weapons.mine.dim;
    while positions.len() < player.weapons.mines as usize {
        let x = thread_rng()
            .gen_range(-SIZE.x * 0.5 + size.x..=SIZE.x * 0.5 - WEAPONS_PANEL_SIZE.x - size.x);
        let y = thread_rng().gen_range(
            -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 1.8
                ..=SIZE.y * 0.5 - MENU_PANEL_SIZE.y - FOW_SIZE.y - size.y,
        );
        let pos = Vec2::new(x, y);

        // Check mines spawn at a minimum distance from each other
        if positions
            .iter()
            .all(|&v| pos.distance(v) >= 2. * size.length())
        {
            positions.push(pos);

            commands.spawn((
                Sprite {
                    image: asset_server.load(weapons.mine.image),
                    custom_size: Some(weapons.mine.dim),
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 1.2),
                weapons.mine.clone(),
            ));
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut weapon_q: Query<(&mut Transform, &mut Weapon), With<Weapon>>,
    enemy_q: Query<(Entity, &Transform, &Enemy), (With<Enemy>, Without<Weapon>)>,
    fence_q: Query<(&Transform, &Sprite), (With<Fence>, Without<Weapon>)>,
    wall_q: Query<(&Transform, &Sprite), (With<Wall>, Without<Weapon>)>,
    fow_q: Query<&Transform, (With<FogOfWar>, Without<Weapon>)>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    for (mut transform, mut weapon) in weapon_q.iter_mut() {
        // Select a target in range
        if let Some((enemy_t, enemy)) = weapon.get_lock(&transform, &enemy_q, &fow_q, &player) {
            if player.resources >= weapon.fire_cost {
                let mut bullet = weapon.bullet.clone();

                let mut d = -(enemy_t.translation - transform.translation);

                // If smart weapons technology is researched, predict enemy movement
                if player.technology.movement_prediction {
                    // No need to take game speed into account since
                    // the effect cancels out on enemy and bullet speed
                    let mut next_pos = enemy_t.translation
                        - Vec3::new(0., enemy.speed * d.length() / bullet.speed, 0.);

                    // If there's a structure, stop movement there
                    if let Some((t, fence)) = fence_q.iter().next() {
                        let fence_y = t.translation.y + fence.custom_size.unwrap().y * 0.5 + 5.;
                        if next_pos.y < fence_y {
                            next_pos.y = fence_y;
                        }
                    } else if let Some((t, wall)) = wall_q.iter().next() {
                        let wall_y = t.translation.y + wall.custom_size.unwrap().y * 0.5 + 5.;
                        if next_pos.y < wall_y {
                            next_pos.y = wall_y;
                        }
                    }

                    d = next_pos - transform.translation;
                }

                let angle = d.y.atan2(d.x);
                bullet.angle = angle;

                // Rotate the weapon towards the selected target
                if weapon.is_aiming(&angle, &transform) {
                    // Check if the weapon can fire (fire timer is finished)
                    if weapon.can_fire(&time, &game_settings) {
                        // Release target lock
                        weapon.lock = None;

                        if matches!(weapon.fire_strategy, FireStrategy::Density { .. }) {
                            bullet.max_distance = d.length() - bullet.dim.length();
                        }

                        // Spawn fire animation
                        let atlas = assets.get_atlas(&weapon.fire_animation.atlas);
                        commands.spawn((
                            Sprite {
                                image: atlas.image,
                                texture_atlas: Some(atlas.texture),
                                ..default()
                            },
                            Transform {
                                translation: Vec3::new(
                                    transform.translation.x
                                        + weapon.dim.x
                                            * weapon.fire_animation.scale.x
                                            * angle.cos(),
                                    transform.translation.y
                                        + weapon.dim.y
                                            * weapon.fire_animation.scale.x
                                            * angle.sin(),
                                    4.0,
                                ),
                                rotation: Quat::from_rotation_z(bullet.angle),
                                scale: weapon.fire_animation.scale,
                                ..default()
                            },
                            AnimationComponent {
                                timer: Timer::from_seconds(
                                    weapon.fire_animation.duration,
                                    TimerMode::Repeating,
                                ),
                                last_index: atlas.last_index,
                                explosion: None,
                            },
                        ));

                        commands.spawn((
                            Sprite {
                                image: asset_server.load(bullet.image),
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
    mut player: ResMut<Player>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut night_stats: ResMut<NightStats>,
    assets: Local<WorldAssets>,
) {
    for (mut transform, entity, mut bullet) in bullet_q.iter_mut() {
        let dx = bullet.speed * bullet.angle.cos() * settings.speed * time.delta_secs();
        let dy = bullet.speed * bullet.angle.sin() * settings.speed * time.delta_secs();

        let d_pos = Vec3::new(dx, dy, 0.);
        transform.translation += d_pos;

        bullet.distance += d_pos.length();

        match &bullet.detonation {
            Detonation::SingleTarget(d) | Detonation::Piercing(d) => {
                for (transform_enemy, enemy_entity, mut enemy) in enemy_q.iter_mut() {
                    // If the bullet can hit grounded/airborne enemies
                    // and the bullet collided with an enemy -> despawn and resolve
                    if ((d.ground > 0. && !enemy.can_fly) || (d.air > 0. && enemy.can_fly))
                        && collision(
                            &transform.translation,
                            &bullet.dim,
                            &transform_enemy.translation,
                            &enemy.dim,
                        )
                    {
                        // Piercing bullets don't despawn on impact
                        if matches!(bullet.detonation, Detonation::SingleTarget { .. }) {
                            commands.entity(entity).try_despawn();
                        }

                        resolve_enemy_impact(
                            &mut commands,
                            d,
                            enemy_entity,
                            &mut enemy,
                            &mut night_stats,
                        );

                        break;
                    }
                }
            }
            Detonation::OnHitExplosion(e) => {
                for (transform_enemy, _, enemy) in enemy_q.iter_mut() {
                    // If the bullet can hit grounded/airborne enemies
                    // and the bullet collided with an enemy -> despawn and resolve
                    if ((e.damage.ground > 0. && !enemy.can_fly)
                        || (e.damage.air > 0. && enemy.can_fly))
                        && collision(
                        &transform.translation,
                        &bullet.dim,
                        &transform_enemy.translation,
                        &enemy.dim,
                    )
                    {
                        // Only mines have 0 speed
                        if bullet.speed == 0. {
                            player.weapons.mines -= 1;
                        }

                        spawn_explosion(&mut commands, &entity, &transform, e, &assets);
                        break;
                    }
                }
            }
            Detonation::OnLocationExplosion(e) if bullet.distance >= bullet.max_distance => {
                spawn_explosion(&mut commands, &entity, &transform, e, &assets);
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
            commands.entity(entity).try_despawn();
        }
    }
}

pub fn update_resources(
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

    let spotlight_cost = &player.spotlight.cost
        * player.spotlight.power as f32
        * game_settings.speed
        * time.delta_secs();

    if player.resources >= spotlight_cost {
        player.resources -= &spotlight_cost;
    } else {
        player.spotlight.power = 0;
    }
}

pub fn resolve_enemy_impact(
    commands: &mut Commands,
    damage: &Damage,
    enemy_entity: Entity,
    enemy: &mut Enemy,
    night_stats: &mut NightStats,
) {
    let damage = damage.calculate(enemy);
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

pub fn spawn_explosion(
    commands: &mut Commands,
    entity: &Entity,
    transform: &Transform,
    explosion: &Explosion,
    assets: &Local<WorldAssets>,
) {
    commands.entity(*entity).try_despawn();

    let atlas_asset = assets.get_atlas(explosion.atlas);
    commands.spawn((
        Sprite {
            image: atlas_asset.image,
            texture_atlas: Some(atlas_asset.texture),
            custom_size: Some(Vec2::splat(explosion.radius)),
            ..default()
        },
        Transform::from_translation(transform.translation),
        AnimationComponent {
            timer: Timer::from_seconds(explosion.interval, TimerMode::Repeating),
            last_index: atlas_asset.last_index,
            explosion: Some(explosion.clone()),
        },
    ));
}