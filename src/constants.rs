use bevy::math::Vec2;

// Font sizes
pub const NORMAL_FONT_SIZE: f32 = 16.;
pub const LARGE_FONT_SIZE: f32 = 24.;

// Window block sizes (panels and background images)
pub const SIZE: Vec2 = Vec2::new(1400., 900.); // Fix the window size to avoid issues with resizing
pub const WALL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.12);
pub const FENCE_SIZE: Vec2 = Vec2::new(WALL_SIZE.x, WALL_SIZE.y * 0.3);
pub const MENU_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.04);
pub const WEAPONS_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x * 0.203, SIZE.y);
pub const RESOURCES_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x, SIZE.y * 0.05);
pub const MAP_SIZE: Vec2 = Vec2::new(
    SIZE.x - WEAPONS_PANEL_SIZE.x,
    SIZE.y - MENU_PANEL_SIZE.y - RESOURCES_PANEL_SIZE.y,
);

// Game settings
pub const MAX_GAME_SPEED: f32 = 5.;
pub const GAME_SPEED_STEP: f32 = 0.5;

// Night settings
pub const NIGHT_DURATION: f32 = 60.; // In seconds
pub const NO_SPAWN_START: f32 = 0.9; // Starting probability of not spawning enemies per check
pub const NO_SPAWN_STEP: f32 = 0.1; // Probability spawning decrease per night
pub const BETA: f32 = 5.; // Probability factor decrease for above-level enemies

// Weapon settings
pub const MAX_SPOTS: u32 = 10;
pub const MAX_MACHINE_GUN_FIRE_RATE: u32 = 5;
pub const MAX_FLAMETHROWER_POWER: u32 = 5;
pub const MAX_MINES: u32 = 25;
