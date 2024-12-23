use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;

pub struct EnemyStatus {
    pub alive: u32,
    pub killed: u32,
}

#[derive(Resource)]
pub struct WaveStats {
    pub enemies: HashMap<String, EnemyStatus>,
}

impl Default for WaveStats {
    fn default() -> Self {
        Self {
            enemies: HashMap::default(),
        }
    }
}