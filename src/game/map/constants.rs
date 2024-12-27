use bevy::math::Vec2;

pub const SIZE: Vec2 = Vec2::new(1280., 800.); // Fix the window size to avoid issues with resizing
pub const WALL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.1);
pub const WEAPONS_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x * 0.2, SIZE.y - RESOURCES_PANEL_SIZE.y);
pub const RESOURCES_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x, SIZE.y * 0.1);
pub const MAP_SIZE: Vec2 = Vec2::new(
    SIZE.x - WEAPONS_PANEL_SIZE.x,
    SIZE.y - WALL_SIZE.y - RESOURCES_PANEL_SIZE.y,
);
