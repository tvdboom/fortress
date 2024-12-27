use crate::resources::Resources;
use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;

pub struct EnemyStatus {
    pub spawned: u32,
    pub killed: u32,
}

#[derive(Resource)]
pub struct WaveStats {
    pub wave: u32,
    pub resources: Resources,
    pub enemies: HashMap<String, EnemyStatus>,
}

impl Default for WaveStats {
    fn default() -> Self {
        Self {
            wave: 1,
            resources: Resources::default(),
            enemies: HashMap::default(),
        }
    }
}
