use crate::constants::{SpriteQ, STRUCTURE_OFFSET};
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::Enemy;
use crate::game::map::components::AnimationComponent;
use crate::game::resources::NightStats;
use crate::game::weapon::components::{Damage, Explosion};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

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

/// Resolve the impact/collision of a bullet on an enemy
pub fn resolve_impact(
    commands: &mut Commands,
    enemy_e: Entity,
    enemy: &mut Enemy,
    damage: &Damage,
    night_stats: &mut NightStats,
) {
    let damage = damage.calculate(enemy);
    if enemy.health <= damage {
        commands.entity(enemy_e).despawn_recursive();

        night_stats
            .enemies
            .entry(enemy.name)
            .and_modify(|status| status.killed += 1);
    } else {
        enemy.health -= damage;
    }
}

/// Spawn an explosion sprite
pub fn spawn_explosion(
    commands: &mut Commands,
    entity: &Entity,
    transform: &Transform,
    explosion: &Explosion,
    assets: &Local<WorldAssets>,
) {
    commands.entity(*entity).try_despawn();

    let atlas_asset = assets.get_atlas(explosion.atlas);
    commands.spawn((
        Sprite {
            image: atlas_asset.image,
            texture_atlas: Some(atlas_asset.texture),
            custom_size: Some(Vec2::splat(explosion.radius)),
            ..default()
        },
        Transform::from_translation(transform.translation),
        AnimationComponent {
            timer: Timer::from_seconds(explosion.interval, TimerMode::Repeating),
            last_index: atlas_asset.last_index,
            explosion: Some(explosion.clone()),
        },
    ));
}
