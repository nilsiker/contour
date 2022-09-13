use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use std::{fs, io::Write, path::PathBuf};

fn get_path() -> PathBuf {
    std::env::current_dir()
        .expect("Could not access current work directory")
        .join("settings.json")
}

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VideoSettings::from_config())
            .insert_resource(AudioSettings::from_config())
            .add_event::<ConfigUpdateEvent>()
            .add_system(save_config_to_file);
    }
}

pub struct VideoSettings {
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: (f32, f32),
}
impl VideoSettings {
    pub fn from_config() -> Self {
        let settings = fetch_settings(get_path()).unwrap();
        VideoSettings {
            vsync: settings.vsync,
            fullscreen: settings.fullscreen,
            resolution: settings.resolution,
        }
    }
}
impl FromWorld for VideoSettings {
    fn from_world(_: &mut World) -> Self {
        Self::from_config()
    }
}
pub struct AudioSettings {
    pub bgm: f64,
    pub sfx: f64,
}

impl AudioSettings {
    pub fn from_config() -> Self {
        let settings = fetch_settings(get_path()).unwrap();
        AudioSettings {
            bgm: settings.bgm,
            sfx: settings.sfx,
        }
    }
}
impl FromWorld for AudioSettings {
    fn from_world(_: &mut World) -> Self {
        Self::from_config()
    }
}

#[derive(Serialize, Deserialize)]
struct Settings {
    bgm: f64,
    sfx: f64,
    vsync: bool,
    fullscreen: bool,
    resolution: (f32, f32),
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            bgm: 50.0,
            sfx: 50.0,
            resolution: (1920.0, 1080.0),
            vsync: false,
            fullscreen: false,
        }
    }
}

pub struct ConfigUpdateEvent;

fn save_config_to_file(
    video: Res<VideoSettings>,
    audio: Res<AudioSettings>,
    events: EventReader<ConfigUpdateEvent>,
) {
    if !events.is_empty() {
        events.clear();
        let settings_path = std::env::current_dir()
            .expect("Could not access current work directory")
            .join("settings.json");
        let mut settings = match fetch_settings(settings_path.clone()) {
            Ok(settings) => settings,
            Err(_) => Settings::default(),
        };

        if video.is_changed() {
            settings.fullscreen = video.fullscreen;
            settings.vsync = video.vsync;
            settings.resolution = video.resolution;
        }
        if audio.is_changed() {
            settings.bgm = audio.bgm;
            settings.sfx = audio.sfx;
        }

        let mut file = fs::File::create(settings_path).expect("Unable to create file");
        let new_settings = serde_json::to_string_pretty::<Settings>(&settings)
            .expect("Failed to parse settings into file format");
        match write!(&mut file, "{}", new_settings) {
            Ok(_) => bevy::log::info!("saved to config"),
            Err(_) => bevy::log::error!("failed to write to config"),
        }
    }
}

fn fetch_settings(path: PathBuf) -> Result<Settings, &'static str> {
    match fs::read_to_string(path) {
        Ok(data) => match serde_json::from_str::<Settings>(data.as_str()) {
            Ok(settings) => Ok(settings),
            Err(_) => Ok(Settings::default()),
        },
        Err(_) => Err("Could not find file"),
    }
}
