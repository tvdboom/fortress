use crate::constants::*;
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::utils::get_future_position;
use crate::game::map::components::{AnimationComponent, FogOfWar};
use crate::game::map::utils::collision;
use crate::game::resources::{GameSettings, NightStats, Player, TechnologyName};
use crate::game::weapon::components::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::f32::consts::PI;

pub fn spawn_fence(
    commands: &mut Commands,
    fence_q: &Query<SpriteQ, With<FenceComponent>>,
    player: &Player,
    asset_server: &AssetServer,
) {
    if player.fence.health > 0. {
        // Despawn existing since we can require a new image (more fence lines)
        if let Ok((entity, _, _)) = fence_q.get_single() {
            commands.entity(entity).despawn();
        }

        let level = (1 + player.fence.max_health as u32 / 1000).min(3);
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
            FenceComponent,
        ));
    }
}

pub fn spawn_wall(
    commands: &mut Commands,
    wall_q: &Query<SpriteQ, With<WallComponent>>,
    player: &Player,
    asset_server: &AssetServer,
) {
    if player.wall.health > 0. && wall_q.get_single().is_err() {
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
            WallComponent,
        ));
    }
}

pub fn spawn_spots(
    commands: &mut Commands,
    weapon_q: &Query<Entity, With<Weapon>>,
    player: &Player,
    weapons: &WeaponManager,
    asset_server: &AssetServer,
) {
    for entity in weapon_q.iter() {
        commands.entity(entity).despawn();
    }

    let positions = player
        .weapons
        .spots
        .iter()
        .enumerate()
        .map(|(i, _)| (i + 1) as f32 * MAP_SIZE.x / (player.weapons.spots.len() + 1) as f32)
        .collect::<Vec<f32>>();

    for (spot, pos) in player.weapons.spots.iter().zip(positions) {
        if let Some(w) = spot.weapon {
            let mut w = weapons.get(&w);
            w.update(&player); // Set the weapon's setting at start

            commands.spawn((
                Sprite {
                    image: asset_server.load(w.image),
                    custom_size: Some(w.dim),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    -SIZE.x * 0.5 + pos,
                    -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + WALL_SIZE.y * 0.5,
                    WEAPON_Z,
                )),
                w,
            ));
        }
    }
}

pub fn spawn_weapons(
    mut commands: Commands,
    fence_q: Query<SpriteQ, With<FenceComponent>>,
    wall_q: Query<SpriteQ, With<WallComponent>>,
    weapon_q: Query<Entity, With<Weapon>>,
    mine_q: Query<SpriteQ, With<Mine>>,
    player: Res<Player>,
    weapons: Res<WeaponManager>,
    asset_server: Res<AssetServer>,
) {
    spawn_fence(&mut commands, &fence_q, &player, &asset_server);
    spawn_wall(&mut commands, &wall_q, &player, &asset_server);
    spawn_spots(&mut commands, &weapon_q, &player, &weapons, &asset_server);

    // Spawn mines
    let mut positions = mine_q
        .iter()
        .map(|(_, t, _)| t.translation.truncate())
        .collect::<Vec<_>>();

    let size = weapons.mine.dim;
    while positions.len() < player.weapons.mines as usize {
        let x = thread_rng()
            .gen_range(-SIZE.x * 0.5 + size.x..=SIZE.x * 0.5 - WEAPONS_PANEL_SIZE.x - size.x);
        let y = thread_rng().gen_range(
            -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y + 2. * WALL_SIZE.y
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
                Mine,
            ));
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut weapon_q: Query<(&mut Transform, &mut Weapon)>,
    enemy_q: Query<EnemyQ, (With<Enemy>, Without<Weapon>)>,
    fence_q: Query<SpriteQ, (With<FenceComponent>, Without<Weapon>)>,
    wall_q: Query<SpriteQ, (With<WallComponent>, Without<Weapon>)>,
    fow_q: Query<&Transform, (With<FogOfWar>, Without<Weapon>)>,
    mut night_stats: ResMut<NightStats>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    assets: Local<WorldAssets>,
    asset_server: Res<AssetServer>,
) {
    for (mut weapon_t, mut weapon) in weapon_q.iter_mut() {
        let mut targets = HashSet::new();

        weapon.target = weapon.acquire_target(&weapon_t, &enemy_q, &fow_q, &targets);
        if let Some(enemy_e) = weapon.target {
            let (_, enemy_t, enemy) = enemy_q.get(enemy_e).unwrap();

            // Determine the bullet's angle towards the target
            let d = -weapon_t.translation
                + (if player.has_tech(TechnologyName::Aimbot)
                    && !matches!(weapon.bullet.movement, Movement::Homing(_))
                {
                    get_future_position(
                        enemy_t.translation,
                        enemy.speed,
                        weapon_t.translation,
                        weapon.bullet.speed,
                        fence_q.get_single(),
                        wall_q.get_single(),
                    )
                } else {
                    enemy_t.translation
                });
            let angle = d.y.atan2(d.x);

            // Check if the player has enough resources to fire
            if player.resources >= weapon.bullet.price && weapon.n_bullets > 0 {
                // Check if the weapon points towards the first target
                if weapon.is_aiming(&angle, &weapon_t) {
                    // Check if the weapon can fire (fire timer is finished)
                    if weapon.can_fire(&time, &game_settings) {
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
                                rotation: Quat::from_rotation_z(angle),
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

                        for i in 0..weapon.n_bullets {
                            if player.resources >= weapon.bullet.price {
                                let mut bullet = weapon.bullet.clone();

                                night_stats.resources += &bullet.price;
                                player.resources -= &bullet.price;

                                // Special case => turret only fires with ui button
                                if weapon.name == WeaponName::Turret {
                                    let power = 1.05
                                        + 0.01
                                            * player
                                                .weapons
                                                .upgrades
                                                .get(&weapon.name)
                                                .unwrap_or(&(0, 0))
                                                .0
                                                as f32;

                                    // The damage increases exponentially with the firepower
                                    bullet.impact = Impact::SingleTarget(Damage {
                                        ground: power.powf(player.weapons.settings.turret),
                                        air: power.powf(player.weapons.settings.turret),
                                        penetration: power.powf(player.weapons.settings.turret),
                                    });
                                    player.weapons.settings.turret = 0.;
                                    weapon.fire_strategy = FireStrategy::None;
                                }

                                // From the 2nd bullet onwards, update the target
                                let (enemy_e, enemy_t, enemy) = match i {
                                    i if i > 0 => {
                                        let enemy_e = weapon
                                            .acquire_target(&weapon_t, &enemy_q, &fow_q, &targets)
                                            .unwrap_or(enemy_e);
                                        enemy_q.get(enemy_e).unwrap()
                                    }
                                    _ => (enemy_e, enemy_t, enemy),
                                };

                                targets.insert(enemy_e);

                                // Determine the bullet's movement
                                match bullet.movement {
                                    Movement::Location(_) => {
                                        bullet.movement = Movement::Location(
                                            if player.has_tech(TechnologyName::Aimbot) {
                                                get_future_position(
                                                    enemy_t.translation,
                                                    enemy.speed,
                                                    weapon_t.translation,
                                                    weapon.bullet.speed,
                                                    fence_q.get_single(),
                                                    wall_q.get_single(),
                                                )
                                            } else {
                                                enemy_t.translation
                                            },
                                        );
                                    }
                                    Movement::Homing(_) => {
                                        bullet.movement = Movement::Homing(enemy_e)
                                    }
                                    _ => (),
                                }

                                commands.spawn((
                                    Sprite {
                                        image: asset_server.load(bullet.image),
                                        custom_size: Some(bullet.dim),
                                        ..default()
                                    },
                                    Transform {
                                        translation: Vec3::new(
                                            weapon_t.translation.x
                                                + weapon.dim.x * 0.5 * angle.cos(),
                                            weapon_t.translation.y
                                                + weapon.dim.y * 0.5 * angle.sin(),
                                            3.0,
                                        ),
                                        rotation: Quat::from_rotation_z(angle),
                                        ..default()
                                    },
                                    bullet,
                                ));
                            }

                            // Reset target lock
                            weapon.target = None;
                        }
                    }
                } else {
                    // Not pointing at target -> rotate towards it
                    weapon_t.rotation = weapon_t.rotation.rotate_towards(
                        Quat::from_rotation_z(angle - PI * 0.5),
                        weapon.rotation_speed * game_settings.speed * time.delta_secs(),
                    );
                }

                continue;
            }
        }

        // If it didn't find a target or doesn't have the
        // resources to fire, return to the default position
        weapon_t.rotation = weapon_t.rotation.slerp(
            Quat::from_rotation_z(0.),
            weapon.rotation_speed * game_settings.speed * time.delta_secs(),
        );
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut bullet_q: Query<(Entity, &mut Transform, &mut Bullet)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Enemy), Without<Bullet>>,
    mut player: ResMut<Player>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
    assets: Local<WorldAssets>,
) {
    for (bullet_e, mut bullet_t, mut bullet) in bullet_q.iter_mut() {
        // Calculate the new target's position wrt the bullet
        let d = match bullet.movement {
            Movement::Straight => None,
            Movement::Location(v) => Some(v - bullet_t.translation),
            Movement::Homing(enemy_e) => {
                if let Ok((_, enemy_t, _)) = enemy_q.get(enemy_e) {
                    Some(enemy_t.translation - bullet_t.translation)
                } else {
                    // If the target doesn't exist anymore, despawn the bullet
                    commands.entity(bullet_e).try_despawn();
                    continue;
                }
            }
        };

        // Turn the bullet's rotation towards the target's position
        if let Some(d) = d {
            bullet_t.rotation = bullet_t.rotation.rotate_towards(
                Quat::from_rotation_z(d.y.atan2(d.x)),
                8. * game_settings.speed * time.delta_secs(),
            );
        }

        // Move the bullet in the direction it's pointing
        let d_pos = (bullet_t.rotation * Vec3::X).normalize()
            * bullet.speed
            * game_settings.speed
            * time.delta_secs();

        bullet_t.translation += d_pos;
        bullet.distance += d_pos.length();

        // If the bullet is at target -> resolve
        match bullet.movement {
            Movement::Straight => {
                for (enemy_e, enemy_t, mut enemy) in enemy_q.iter_mut() {
                    // If the bullet can hit grounded/airborne enemies
                    // and the bullet collided with an enemy -> despawn and resolve
                    if collision(
                        &bullet_t.translation,
                        &bullet.dim,
                        &enemy_t.translation,
                        &enemy.dim,
                    ) {
                        // Special case: mines are only triggered by a certain size
                        // (only mines have 0 speed)
                        if bullet.speed == 0. && enemy.size < player.weapons.settings.mine {
                            continue;
                        }

                        let impacted = bullet.impact.resolve(
                            &mut commands,
                            bullet_e,
                            &bullet_t,
                            Some((enemy_e, &mut enemy)),
                            &assets,
                        );

                        // Update mine counts
                        if impacted && bullet.speed == 0. {
                            player.weapons.mines -= 1;
                        }
                    }
                }
            }
            Movement::Location(v) => {
                // Accept a 0.5% error margin
                if bullet_t.translation.distance(v) <= MAP_SIZE.y * 0.005 {
                    bullet
                        .impact
                        .resolve(&mut commands, bullet_e, &bullet_t, None, &assets);
                }
            }
            Movement::Homing(enemy_e) => {
                let (enemy_e, enemy_t, mut enemy) = enemy_q.get_mut(enemy_e).unwrap();

                if collision(
                    &bullet_t.translation,
                    &bullet.dim,
                    &enemy_t.translation,
                    &enemy.dim,
                ) {
                    bullet.impact.resolve(
                        &mut commands,
                        bullet_e,
                        &bullet_t,
                        Some((enemy_e, &mut enemy)),
                        &assets,
                    );
                }
            }
        }

        // If the bullet traveled more than max distance or left window boundaries -> despawn
        if bullet.max_distance != f32::MAX
            && (bullet.distance >= bullet.max_distance
                || bullet_t.translation.x < -SIZE.x * 0.5
                || bullet_t.translation.x > SIZE.x * 0.5 - WEAPONS_PANEL_SIZE.x
                || bullet_t.translation.y > SIZE.y * 0.5 - MENU_PANEL_SIZE.y
                || bullet_t.translation.y < -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y)
        {
            commands.entity(bullet_e).try_despawn();
        }
    }
}
