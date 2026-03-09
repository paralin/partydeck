use crate::paths::*;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

/// A single saved instance: which devices (by stable ID) and which profile.
#[derive(Serialize, Deserialize, Clone)]
pub struct SavedInstance {
    pub device_ids: Vec<String>,
    pub profile: String,
}

/// Last instance configuration for a handler, saved between sessions.
#[derive(Serialize, Deserialize, Clone)]
pub struct SavedInstanceConfig {
    pub handler_name: String,
    pub instances: Vec<SavedInstance>,
}

pub fn load_last_instances(handler_name: &str) -> Option<SavedInstanceConfig> {
    let path = PATH_PARTY.join("last_instances.json");
    let file = File::open(path).ok()?;
    let config: SavedInstanceConfig = serde_json::from_reader(BufReader::new(file)).ok()?;
    if config.handler_name == handler_name {
        Some(config)
    } else {
        None
    }
}

pub fn save_last_instances(config: &SavedInstanceConfig) -> Result<(), Box<dyn Error>> {
    let path = PATH_PARTY.join("last_instances.json");
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum PadFilterType {
    All,
    #[default]
    NoSteamInput,
    OnlySteamInput,
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PartyConfig {
    #[serde(default = "default_true")]
    pub enable_kwin_script: bool,
    #[serde(default = "default_true")]
    pub gamescope_fix_lowres: bool,
    #[serde(default = "default_true")]
    pub gamescope_sdl_backend: bool,
    #[serde(default)]
    pub gamescope_force_grab_cursor: bool,
    #[serde(default = "default_true")]
    pub kbm_support: bool,
    #[serde(default)]
    pub proton_version: String,
    #[serde(default = "default_true")]
    pub proton_separate_pfxs: bool,
    #[serde(default = "default_true")]
    pub proton_wow64: bool,
    #[serde(default)]
    pub vertical_two_player: bool,
    #[serde(default)]
    pub pad_filter_type: PadFilterType,
    #[serde(default)]
    pub allow_multiple_instances_on_same_device: bool,
    #[serde(default = "default_true")]
    pub profile_unique_dirs: bool,
    #[serde(default)]
    pub disable_mount_gamedirs: bool,
}

impl Default for PartyConfig {
    fn default() -> Self {
        PartyConfig {
            enable_kwin_script: true,
            gamescope_fix_lowres: true,
            gamescope_sdl_backend: true,
            gamescope_force_grab_cursor: false,
            kbm_support: true,
            proton_version: "".to_string(),
            proton_separate_pfxs: true,
            proton_wow64: true,
            vertical_two_player: false,
            pad_filter_type: PadFilterType::NoSteamInput,
            allow_multiple_instances_on_same_device: false,
            profile_unique_dirs: true,
            disable_mount_gamedirs: false,
        }
    }
}

pub fn load_cfg() -> PartyConfig {
    let path = PATH_PARTY.join("settings.json");

    if let Ok(file) = File::open(path) {
        if let Ok(config) = serde_json::from_reader::<_, PartyConfig>(BufReader::new(file)) {
            return config;
        }
    }

    // Return default settings if file doesn't exist or has error
    return PartyConfig::default();
}

pub fn save_cfg(config: &PartyConfig) -> Result<(), Box<dyn Error>> {
    let path = PATH_PARTY.join("settings.json");
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}
