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

/// Calculate the distance between an enemy and a bullet.
/// If movement_prediction is true, the distance to the future position of the enemy is calculated.
pub fn calculate_distance(
    enemy: &Enemy,
    enemy_pos: &Vec3,
    bullet: &Bullet,
    bullet_pos: &Vec3,
    fence_q: Result<SpriteQ, QuerySingleError>,
    wall_q: Result<SpriteQ, QuerySingleError>,
    movement_prediction: bool,
) -> Vec3 {
    let mut d = -(enemy_pos - bullet_pos);

    // Predict enemy movement comes with a technology
    if movement_prediction {
        // No need to take game speed into account since
        // the effect cancels out on enemy and bullet speed
        let mut next_pos = enemy_pos - Vec3::new(0., enemy.speed * d.length() / bullet.speed, 0.);

        // If there's a structure, stop movement there
        if let Some(fence_y) = get_structure_top(fence_q) {
            if next_pos.y < fence_y {
                next_pos.y = fence_y;
            }
        } else if let Some(wall_y) = get_structure_top(wall_q) {
            if next_pos.y < wall_y {
                next_pos.y = wall_y;
            }
        }

        d = next_pos - bullet_pos
    }

    d
}
