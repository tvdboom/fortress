use crate::constants::SpriteQ;
use crate::game::weapon::utils::get_structure_top;
use bevy::ecs::query::QuerySingleError;
use bevy::math::Vec3;

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
