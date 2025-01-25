use crate::game::resources::{GameSettings, Player};
use crate::game::AppState;
use crate::messages::Messages;
use bevy::prelude::{Commands, NextState};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{fs, io};

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

#[derive(Serialize, Deserialize)]
pub struct SaveAll {
    pub player: Player,
    pub speed: f32,
    pub audio: bool,
}

/// Trait to get the name of an enum variant
pub trait NameFromEnum {
    fn name(&self) -> String;
}

impl<T: Debug> NameFromEnum for T {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

/// Scale a Duration by a factor
pub fn scale_duration(duration: Duration, scale: f32) -> Duration {
    let sec = (duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9) * scale;
    Duration::new(sec.trunc() as u64, (sec.fract() * 1e9) as u32)
}

fn save_to_json(file_path: &str, data: &SaveAll) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(data)?;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

fn load_from_json(file_path: &str) -> io::Result<SaveAll> {
    let json_data = fs::read_to_string(file_path)?;
    let data: SaveAll = serde_json::from_str(&json_data)?;
    Ok(data)
}

/// Load a game from a JSON file
#[cfg(not(target_arch = "wasm32"))]
pub fn load_game(
    commands: &mut Commands,
    game_settings: &GameSettings,
    next_app_state: &mut NextState<AppState>,
    messages: &mut Messages,
) {
    if let Some(file_path) = FileDialog::new().pick_file() {
        let file_path_str = file_path.to_string_lossy().to_string();
        let data = load_from_json(&file_path_str).expect("Failed to load the game.");

        commands.insert_resource(data.player);
        commands.insert_resource(GameSettings {
            speed: data.speed,
            audio: data.audio,
            just_loaded: true,
            ..game_settings.clone()
        });
        next_app_state.set(AppState::Day);
        messages.info("Game loaded.");
    }
}

/// Save the game to a JSON file
#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(player: &Player, game_settings: &GameSettings, messages: &mut Messages) {
    if let Some(mut file_path) = FileDialog::new().save_file() {
        if !file_path.extension().map(|e| e == "json").unwrap_or(false) {
            file_path.set_extension("json");
        }

        let file_path_str = file_path.to_string_lossy().to_string();
        let data = SaveAll {
            player: player.clone(),
            speed: game_settings.speed,
            audio: game_settings.audio,
        };

        save_to_json(&file_path_str, &data).expect("Failed to save the game.");
        messages.info("Game saved.");
    }
}
