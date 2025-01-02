use bevy::math::Vec2;

// Font sizes
pub const NORMAL_FONT_SIZE: f32 = 16.;
pub const LARGE_FONT_SIZE: f32 = 24.;

// Window block sizes (panels and background images)
pub const SIZE: Vec2 = Vec2::new(1400., 900.); // Fix the window size to avoid issues with resizing
pub const WALL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.12);
pub const MENU_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.04);
pub const WEAPONS_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x * 0.203, SIZE.y);
pub const RESOURCES_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x, SIZE.y * 0.05);
pub const MAP_SIZE: Vec2 = Vec2::new(
    SIZE.x - WEAPONS_PANEL_SIZE.x,
    SIZE.y - MENU_PANEL_SIZE.y - RESOURCES_PANEL_SIZE.y,
);

pub const NIGHT_DURATION: f32 = 60.;
pub const MAX_GAME_SPEED: f32 = 5.;
pub const GAME_SPEED_STEP: f32 = 0.5;
