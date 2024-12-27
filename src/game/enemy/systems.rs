use super::components::*;
use crate::game::components::*;
use crate::game::map::components::*;
use crate::game::map::constants::{MAP_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::resources::{EnemyStatus, WaveStats};
use crate::game::systems::pause_game;
use crate::game::GameState;
use crate::resources::Player;
use bevy::color::{
    palettes::basic::{BLACK, LIME},
    Color,
};
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    mut wave_stats: ResMut<WaveStats>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = thread_rng();
    let enemy = match rng.gen_range(0..1000) * wave_stats.wave {
        800..950 => Enemy::walker(),
        950..990 => Enemy::runner(),
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
            Transform::from_xyz(x, (SIZE.y + enemy.size.y) * 0.5, 2.0),
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

    wave_stats
        .enemies
        .entry(enemy.name.clone())
        .and_modify(|status| status.spawned += 1)
        .or_insert_with(|| EnemyStatus {
            spawned: 1,
            killed: 0,
        });
}

pub fn move_enemies(
    mut enemy_q: Query<(&mut Transform, &Enemy)>,
    wall_q: Query<(&Transform, &Sprite), (With<Wall>, Without<Enemy>)>,
    vis_q: Query<&mut Visibility, With<PauseWrapper>>,
    mut player: ResMut<Player>,
    next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let (t, wall) = wall_q.iter().next().unwrap();
    let wall_y = t.translation.y + wall.custom_size.unwrap().y * 0.5;

    for (mut transform, enemy) in enemy_q.iter_mut() {
        let new_pos = transform.translation.y - MAP_SIZE.y / 100. * enemy.speed * time.delta_secs();

        if new_pos < wall_y + 5. {
            transform.translation.y = wall_y + 5.;

            if player.wall.health > enemy.damage {
                player.wall.health -= enemy.damage;
            } else {
                pause_game(vis_q, next_state);
                todo!();
            }
        } else {
            transform.translation.y = new_pos;
        }
    }
}
