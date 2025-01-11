use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::utils::calculate_distance;
use crate::game::map::components::{AnimationComponent, FogOfWar};
use crate::game::map::utils::collision;
use crate::game::resources::{GameSettings, NightStats, Player};
use crate::game::weapon::components::*;
use crate::game::weapon::utils::{resolve_impact, spawn_explosion};
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
                STRUCTURE_Z,
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
            STRUCTURE_Z,
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
                        WEAPON_Z,
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
                Transform::from_xyz(pos.x, pos.y, STRUCTURE_Z),
                weapons.mine.clone(),
            ));
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut weapon_q: Query<(&mut Transform, &mut Weapon)>,
    enemy_q: Query<EnemyQ, (With<Enemy>, Without<Weapon>)>,
    fence_q: Query<SpriteQ, (With<Fence>, Without<Weapon>)>,
    wall_q: Query<SpriteQ, (With<Wall>, Without<Weapon>)>,
    fow_q: Query<SpriteQ, (With<FogOfWar>, Without<Weapon>)>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    'w: for (mut weapon_t, mut weapon) in weapon_q.iter_mut() {
        if let Some(targets) = weapon.acquire_targets(&weapon_t, &enemy_q, &fow_q, &player) {
            for (i, (enemy_e, enemy_t, enemy)) in targets.iter().enumerate() {
                let mut bullet = weapon.bullet.clone();

                // Homing bullets point at the target and move the angle while flying
                let d = match bullet.movement {
                    Movement::Straight => calculate_distance(
                        enemy,
                        &enemy_t.translation,
                        &bullet,
                        &weapon_t.translation,
                        fence_q.get_single(),
                        wall_q.get_single(),
                        player.technology.movement_prediction,
                    ),
                    Movement::Homing(mut entity) => {
                        entity = *enemy_e;
                        -(enemy_t.translation - weapon_t.translation)
                    },
                };

                let angle = d.y.atan2(d.x);
                bullet.angle = angle;

                // Check if the player has enough resources to fire
                if player.resources >= bullet.price {
                    // Check if the weapon points towards the first target
                    if i > 0 || weapon.is_aiming(&angle, &weapon_t) {
                        // Check if the weapon can fire (fire timer is finished)
                        if weapon.can_fire(&time, &game_settings) {
                            night_stats.resources += &bullet.price;
                            player.resources -= &bullet.price;

                            // Reset targets
                            weapon.target = vec![];

                            if let FireStrategy::Density = weapon.fire_strategy {
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
                                        weapon_t.translation.x
                                            + weapon.dim.x
                                                * weapon.fire_animation.scale.x
                                                * angle.cos(),
                                        weapon_t.translation.y
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
                                        weapon_t.translation.x + weapon.dim.x * 0.5 * angle.cos(),
                                        weapon_t.translation.y + weapon.dim.y * 0.5 * angle.sin(),
                                        3.0,
                                    ),
                                    rotation: Quat::from_rotation_z(bullet.angle),
                                    ..default()
                                },
                                bullet,
                            ));
                        }
                    } else {
                        continue 'w; // The weapon is not pointing towards the target yet
                    }
                }

                if i == 0 {
                    // Rotate only towards the first target
                    weapon_t.rotation = weapon_t.rotation.slerp(
                        Quat::from_rotation_z(angle),
                        weapon.rotation_speed * game_settings.speed * time.delta_secs(),
                    );
                }

                continue 'w; // Not enough resources to fire
            }
        }

        // If the weapon couldn't shoot, return to the default position
        weapon_t.rotation = weapon_t.rotation.slerp(
            Quat::from_rotation_z(PI * 0.5),
            weapon.rotation_speed * game_settings.speed * time.delta_secs(),
        );
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut bullet_q: Query<(Entity, &mut Transform, &mut Bullet)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Enemy), Without<Bullet>>,
    mut player: ResMut<Player>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut night_stats: ResMut<NightStats>,
    assets: Local<WorldAssets>,
) {
    for (bullet_e, mut bullet_t, mut bullet) in bullet_q.iter_mut() {
        if let Movement::Homing(enemy_e) = &bullet.movement {
            if let Ok((_, enemy_t, _)) = enemy_q.get(*enemy_e) {
                let d = -(enemy_t.translation - bullet_t.translation);

                // Move the bullet's angle towards the target's current position
                bullet.angle = d.y.atan2(d.x);
            } else {
                // If the target doesn't exist anymore, despawn the bullet
                commands.entity(bullet_e).try_despawn();
            }
        }

        let dx = bullet.speed * bullet.angle.cos() * settings.speed * time.delta_secs();
        let dy = bullet.speed * bullet.angle.sin() * settings.speed * time.delta_secs();

        let d_pos = Vec3::new(dx, dy, 0.);
        bullet_t.translation += d_pos;

        bullet.distance += d_pos.length();

        match &bullet.impact {
            Impact::SingleTarget(d) | Impact::Piercing(d) => {
                for (enemy_e, enemy_t, mut enemy) in enemy_q.iter_mut() {
                    // If the bullet can hit grounded/airborne enemies
                    // and the bullet collided with an enemy -> despawn and resolve
                    if ((d.ground > 0. && !enemy.can_fly) || (d.air > 0. && enemy.can_fly))
                        && collision(
                            &bullet_t.translation,
                            &bullet.dim,
                            &enemy_t.translation,
                            &enemy.dim,
                        )
                    {
                        // Piercing bullets don't despawn on impact
                        if let Impact::SingleTarget(_) = bullet.impact {
                            commands.entity(bullet_e).try_despawn();
                        }

                        resolve_impact(&mut commands, enemy_e, &mut enemy, d, &mut night_stats);
                        break;
                    }
                }
            }
            Impact::OnHitExplosion(e) => {
                for (_, enemy_t, enemy) in enemy_q.iter_mut() {
                    // If the bullet can hit grounded/airborne enemies
                    // and the bullet collided with an enemy -> despawn and resolve
                    if ((e.damage.ground > 0. && !enemy.can_fly)
                        || (e.damage.air > 0. && enemy.can_fly))
                        && collision(
                            &bullet_t.translation,
                            &bullet.dim,
                            &enemy_t.translation,
                            &enemy.dim,
                        )
                    {
                        // Only mines have 0 speed
                        if bullet.speed == 0. {
                            player.weapons.mines -= 1;
                        }

                        spawn_explosion(&mut commands, &bullet_e, &bullet_t, e, &assets);
                        break;
                    }
                }
            }
            Impact::OnLocationExplosion(e) if bullet.distance >= bullet.max_distance => {
                spawn_explosion(&mut commands, &bullet_e, &bullet_t, e, &assets);
                break;
            }
            Impact::OnTargetExplosion(e) => {
                if let Movement::Homing(enemy_e) = bullet.movement {
                    if let Ok((_, enemy_t, enemy)) = enemy_q.get(*enemy_e) {
                        if collision(
                            &bullet_t.translation,
                            &bullet.dim,
                            &enemy_t.translation,
                            &enemy.dim,
                        ) {
                            spawn_explosion(&mut commands, &bullet_e, &bullet_t, e, &assets);
                            break;
                        }
                    }
                }
            }
            _ => (),
        }

        // If the bullet traveled more than max distance or left window boundaries -> despawn
        if bullet.distance >= bullet.max_distance
            || bullet_t.translation.x < -SIZE.x * 0.5
            || bullet_t.translation.x > SIZE.x * 0.5
            || bullet_t.translation.y > SIZE.y * 0.5
            || bullet_t.translation.y < -SIZE.y * 0.5
        {
            commands.entity(bullet_e).try_despawn();
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
