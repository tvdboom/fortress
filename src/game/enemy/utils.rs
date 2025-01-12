use crate::constants::{EnemyQ, SpriteQ};
use crate::game::enemy::components::Enemy;
use crate::game::weapon::components::Bullet;
use crate::game::weapon::utils::get_structure_top;
use bevy::ecs::query::QuerySingleError;
use bevy::math::Vec3;

pub trait EnemyOperations {
    fn is_visible(&self, fow_q: Result<SpriteQ, QuerySingleError>) -> bool;
}

impl EnemyOperations for EnemyQ<'_> {
    /// Whether the enemy is visible or behind the fog of war
    fn is_visible(&self, fow_q: Result<SpriteQ, QuerySingleError>) -> bool {
        if let Ok((_, fow_t, fow)) = fow_q {
            if fow_t.translation.y
                - fow.custom_size.expect("Fog of war has no custom size.").y * 0.5
                < self.1.translation.y - self.2.dim.y * 0.5
            {
                return false;
            }
        }

        true
    }
}

/// Calculate the future position of an enemy relative to a bullet.
pub fn get_future_position(
    enemy_t: Vec3,
    enemy_speed: f32,
    bullet_t: Vec3,
    bullet_speed: f32,
    fence_q: Result<SpriteQ, QuerySingleError>,
    wall_q: Result<SpriteQ, QuerySingleError>,
) -> Vec3 {
    // No need to take game speed into account since
    // the effect cancels out on enemy and bullet speed
    let yt = enemy_speed * enemy_t.distance(bullet_t) / bullet_speed;
    let mut future_t = enemy_t - Vec3::new(0., yt, 0.);

    // If there's a structure, stop movement there
    if let Some(fence_y) = get_structure_top(fence_q) {
        if future_t.y < fence_y {
            future_t.y = fence_y;
        }
    } else if let Some(wall_y) = get_structure_top(wall_q) {
        if future_t.y < wall_y {
            future_t.y = wall_y;
        }
    }

    future_t
}
