use super::components::*;
use crate::constants::{SpriteQ, ENEMY_Z, RESOURCES_PANEL_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::resources::{EnemyStatus, GameSettings, NightStats, Player};
use crate::game::weapon::components::{Fence, Wall};
use crate::game::weapon::utils::get_structure_top;
use crate::game::AppState;
use crate::messages::Messages;
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
    mut messages: ResMut<Messages>,
    game_settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // Stop spawning enemies when the night timer has finished
    if night_stats.timer.finished() {
        if night_stats.timer.just_finished() {
            messages.info("It's dawning...");
        }

        if enemy_q.iter().count() == 0 {
            next_state.set(AppState::Day);
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
                            Transform::from_xyz(0., enemy.dim.y * 0.5 - 5.0, 0.1),
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
                                Transform::from_xyz(0., 0., 0.2),
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
    mut enemy_q: Query<(&mut Transform, &mut Enemy)>,
    fence_q: Query<SpriteQ, (With<Fence>, Without<Enemy>)>,
    wall_q: Query<SpriteQ, (With<Wall>, Without<Enemy>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut player: ResMut<Player>,
    mut night_stats: ResMut<NightStats>,
    mut messages: ResMut<Messages>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    for (mut enemy_t, mut enemy) in enemy_q.iter_mut() {
        let mut new_pos =
            enemy_t.translation.y - enemy.speed * game_settings.speed * time.delta_secs();

        if !enemy.can_fly {
            if let Some(fence_y) = get_structure_top(fence_q.get_single()) {
                if new_pos < fence_y {
                    new_pos = fence_y;

                    // If the fence is enabled, damage the enemy
                    if player.fence.enabled {
                        enemy.health -=
                            (player.fence.damage * game_settings.speed * time.delta_secs())
                                .min(enemy.health);
                    }

                    player.fence.health -= (enemy.damage * game_settings.speed * time.delta_secs())
                        .min(player.fence.health);
                }
            } else if let Some(wall_y) = get_structure_top(wall_q.get_single()) {
                if new_pos < wall_y {
                    new_pos = wall_y;

                    player.wall.health -= (enemy.damage * game_settings.speed * time.delta_secs())
                        .min(player.wall.health);
                }
            }
        }

        if new_pos < -SIZE.y * 0.5 + RESOURCES_PANEL_SIZE.y - enemy.dim.y * 0.5 {
            messages.error("A bug entered the fortress");

            enemy.health = 0.; // Is despawned in update_game
            let mut damage = enemy.damage as u32;

            // First subtract damage from the soldiers
            let dead_soldiers = ((damage as f32 / 3.) as u32).min(player.population.soldier);
            player.population.soldier -= dead_soldiers;
            night_stats.population.soldier += dead_soldiers;
            damage -= (dead_soldiers * 3).min(damage);

            if damage > 0 {
                // Then randomly over the rest of the population
                // Note that only one type of population can be attacked per bug
                match thread_rng().gen_range(0..4) {
                    0 => {
                        let dead = damage.min(player.population.armorer);
                        player.population.armorer -= dead;
                        night_stats.population.armorer += dead;
                    }
                    1 => {
                        let dead = damage.min(player.population.refiner);
                        player.population.refiner -= dead;
                        night_stats.population.refiner += dead;
                    }
                    2 => {
                        let dead = damage.min(player.population.constructor);
                        player.population.constructor -= dead;
                        night_stats.population.constructor += dead;
                    }
                    3 => {
                        let dead = damage.min(player.population.scientist);
                        player.population.scientist -= dead;
                        night_stats.population.scientist += dead;
                    }
                    _ => {
                        let dead = damage.min(player.population.idle);
                        player.population.idle -= dead;
                        night_stats.population.idle += dead;
                    }
                }
            }

            if player.population.total() == 0 {
                next_state.set(AppState::GameOver);
            }
        } else {
            enemy_t.translation.y = new_pos;
        }
    }
}
