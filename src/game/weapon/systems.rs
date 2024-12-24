use crate::game::enemy::components::Enemy;
use crate::game::resources::WaveStats;
use crate::game::weapon::components::{Bullet, Weapon};
use crate::resources::Player;
use bevy::prelude::*;
use crate::game::map::components::Map;

pub fn spawn_bullets(
    mut commands: Commands,
    mut weapon_q: Query<(&Transform, &mut Weapon)>,
    enemy_q: Query<&Transform, With<Enemy>>,
    map_q: Query<&Sprite, With<Map>>,
    time: Res<Time>,
    mut wave_stats: ResMut<WaveStats>,
    mut player: ResMut<Player>,
    asset_server: Res<AssetServer>,
) {
    let map_height = map_q.get_single().unwrap().custom_size.unwrap().y;

    for (mut transform, mut weapon) in weapon_q.iter_mut() {
        weapon.fire_rate.tick(time.delta());

        // To fire, the following prerequisites must be met:
        // 1. The weapon's timer is finished
        // 2. The player has enough resources to fire
        // 3. There is an enemy in range
        if weapon.fire_rate.finished() {
            if player.resources.bullets > weapon.fire_cost.bullets
                && player.resources.gasoline > weapon.fire_cost.gasoline
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
                    if distance <= map_height / 100. * weapon.bullet.max_distance {
                        // Compute the angle to the nearest enemy
                        let d = nearest_enemy.translation - transform.translation;
                        weapon.bullet.angle = d.y.atan2(d.x);

                        commands.spawn((
                            Sprite {
                                image: asset_server.load(&weapon.bullet.image),
                                custom_size: Some(Vec2::new(
                                    weapon.bullet.size.0,
                                    weapon.bullet.size.1,
                                )),
                                ..default()
                            },
                            Transform {
                                translation: Vec3::new(
                                    transform.translation.x,
                                    transform.translation.y,
                                    3.0,
                                ),
                                rotation: Quat::from_rotation_z(weapon.bullet.angle),
                                ..default()
                            },
                            weapon.bullet.clone(),
                        ));

                        wave_stats.resources.bullets += weapon.fire_cost.bullets;
                        player.resources.bullets -= weapon.fire_cost.bullets;
                    }
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
    mut wave_stats: ResMut<WaveStats>,
    mut player: ResMut<Player>,
    window: Single<&Window>,
) {
    let map_height = map_q.get_single().unwrap().custom_size.unwrap().y;

    for (mut transform, entity, mut bullet) in bullet_q.iter_mut() {
        let dx = map_height / 100. * bullet.speed * bullet.angle.cos() * time.delta_secs();
        transform.translation.x += dx;

        let dy = map_height / 100. * bullet.speed * bullet.angle.sin() * time.delta_secs();
        transform.translation.y += dy;

        // Pythagoras to get distance traveled
        bullet.distance += (dx.powi(2) + dy.powi(2)).sqrt();

        // If outside map -> despawn
        if transform.translation.x < -window.width() * 0.5
            || transform.translation.x > window.width() * 0.5
            || transform.translation.y > window.height() * 0.5
        {
            commands.entity(entity).despawn();
            return;
        }

        // If the bullet collided with an enemy -> resolve and despawn
        // for (transform, enemy_entity, mut enemy) in enemy_q.iter_mut() {
        //     if true {
        //         commands.entity(entity).despawn();
        //
        //         enemy.health -= bullet.damage;
        //     }
        // }

        // If the bullet traveled more than max distance -> despawn
        if bullet.distance >= map_height / 100. * bullet.max_distance {
            commands.entity(entity).despawn();
        }
    }
}
