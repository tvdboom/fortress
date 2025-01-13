use crate::constants::{SpriteQ, STRUCTURE_OFFSET};
use bevy::ecs::query::QuerySingleError;

/// Get the top y coordinate of a structure (fence or wall).
/// Returns None if the structure doesn't exist
pub fn get_structure_top(sprite_q: Result<SpriteQ, QuerySingleError>) -> Option<f32> {
    if let Ok((_, sprite_t, sprite)) = sprite_q {
        let size = sprite.custom_size.expect("Structure has no custom size.");
        Some(sprite_t.translation.y + size.y * 0.5 + STRUCTURE_OFFSET)
    } else {
        None
    }
}
