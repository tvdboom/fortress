use crate::game::enemy::components::Enemy;
use crate::game::weapon::components::{Bullet, Weapon};
use bevy::math::Vec2;
use bevy::prelude::*;

pub type BulletQ<'a> = (Entity, &'a Transform, &'a Bullet);
pub type EnemyQ<'a> = (Entity, &'a Transform, &'a Enemy);
pub type SpriteQ<'a> = (Entity, &'a Transform, &'a Sprite);
pub type WeaponQ<'a> = (Entity, &'a Transform, &'a Weapon);

pub const TITLE: &str = "Fortress";

// General settings
pub const NORMAL_FONT_SIZE: f32 = 16.;
pub const LARGE_FONT_SIZE: f32 = 24.;
pub const MESSAGE_DURATION: u64 = 4; // Seconds that the messages are shown

// Window block sizes (panels and background images)
pub const SIZE: Vec2 = Vec2::new(1440., 834.);
pub const MENU_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x - WEAPONS_PANEL_SIZE.x, SIZE.y * 0.04);
pub const WEAPONS_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x * 0.21, SIZE.y);
pub const RESOURCES_PANEL_SIZE: Vec2 = Vec2::new(SIZE.x, SIZE.y * 0.05);
pub const MAP_SIZE: Vec2 = Vec2::new(
    SIZE.x - WEAPONS_PANEL_SIZE.x,
    SIZE.y - MENU_PANEL_SIZE.y - RESOURCES_PANEL_SIZE.y,
);
pub const WALL_SIZE: Vec2 = Vec2::new(MAP_SIZE.x, SIZE.y * 0.12);
pub const FENCE_SIZE: Vec2 = Vec2::new(MAP_SIZE.x, WALL_SIZE.y * 0.3);
pub const FOW_SIZE: Vec2 = Vec2::new(MAP_SIZE.x, MAP_SIZE.y * 0.3);

// Game settings
pub const MAX_GAME_SPEED: f32 = 5.;
pub const GAME_SPEED_STEP: f32 = 0.5;

// Map settings
pub const STRUCTURE_OFFSET: f32 = 5.; // Distance of structure to stop enemy movement
pub const MAP_Z: f32 = 0.0;
pub const STRUCTURE_Z: f32 = 1.0;
pub const BULLET_Z: f32 = 2.0;
pub const ENEMY_Z: f32 = 3.0;
pub const WEAPON_Z: f32 = 4.0;
pub const EXPLOSION_Z: f32 = 5.0;
pub const FOW_Z: f32 = 6.0;

// Night settings
pub const NIGHT_DURATION: f32 = 45.; // In seconds
pub const NO_SPAWN_START: f32 = 1.0; // Starting probability of not spawning enemies per check
pub const NO_SPAWN_STEP: f32 = 0.05; // Probability spawning decrease per night
pub const BETA: f32 = 5.; // Probability factor decrease for above-level enemies

// Resource settings
pub const POPULATION_MEAN_INCREASE: u32 = 100;
pub const POPULATION_STD_INCREASE: u32 = 10;
pub const SOLDIER_BASE_DAMAGE: u32 = 3;
pub const RESOURCE_FACTOR: f32 = 5.; // Factor to multiply population by to get resources

// Weapon settings
pub const MAX_SPOTS: u32 = 8;
pub const MAX_UPGRADE_LEVEL: u32 = 10;
pub const MAX_MACHINE_GUN_FIRE_RATE: u32 = 5;
pub const MAX_FLAMETHROWER_POWER: u32 = 5;
pub const DEFAULT_TURRET_POWER_TIME: f32 = 10.; // Seconds to power-up the turret
pub const MAX_TURRET_POWER: f32 = 100.;
pub const TURRET_POWER_COST: f32 = 10.; // Gasoline cost per second

pub const MAX_MISSILE_LAUNCHER_SHELLS: u32 = 10;
pub const MAX_SPOTLIGHT_POWER: u32 = 100;
pub const MAX_MINES: u32 = 25;
pub const MAX_BOMBS: u32 = 5;
pub const MAX_NUKES: u32 = 1;
