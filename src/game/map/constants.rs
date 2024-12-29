use bevy::math::Vec2;

// Window block sizes (panels and background images)
pub const SIZE: Vec2 = Vec2::new(1280., 800.); // Fix the window size to avoid issues with resizing
pub const WALL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.12);
pub const MENU_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.04);
pub const WEAPONS_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x * 0.203, SIZE.y);
pub const RESOURCES_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x, SIZE.y * 0.05);
pub const MAP_SIZE: Vec2 = Vec2::new(
    SIZE.x - WEAPONS_PANEL_SIZE.x,
    SIZE.y - MENU_PANEL_SIZE.y - WALL_SIZE.y - RESOURCES_PANEL_SIZE.y,
);

// Font sizes
pub const NORMAL_FONT_SIZE: f32 = 16.;
pub const LARGE_FONT_SIZE: f32 = 24.;
