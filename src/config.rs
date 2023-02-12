use std::{fs::File, io::BufReader, path::Path};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::process::Coord;

static PROGRAM_CONFIG: OnceCell<ProgramConfig> = OnceCell::new();

#[derive(Serialize, Deserialize)]
pub struct ProgramConfig {
    pub run_mode: RunMode,
    pub account: AlertAccount,
    pub rule: AlertRule,
}

impl ProgramConfig {
    pub fn initialize(path: impl AsRef<Path>) {
        let Ok(file) = File::open(path) else {
            panic!("Config file not found.")
        };
        let reader = BufReader::new(file);
        let config = ron::de::from_reader(reader).expect("Config could not be deserialized.");
        let _ = PROGRAM_CONFIG.set(config);
    }

    pub fn get() -> &'static Self {
        PROGRAM_CONFIG.get().expect("Program config is not initialized")
    }
}

#[derive(Default, Serialize, Deserialize)]
pub enum RunMode {
    Debug,
    #[default]
    Dev,
    Release,
}

impl RunMode {
    pub fn is_debug(&self) -> bool {
        matches!(self, Self::Debug)
    }
    
    pub fn is_dev(&self) -> bool {
        matches!(self, Self::Debug)
    }
    
    pub fn is_release(&self) -> bool {
        matches!(self, Self::Debug)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AlertAccount {
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlertRule {
    pub origin: Coord,
    pub search_radius_km_1: f64,
    pub min_magnitude_1: f32,
    pub search_radius_km_2: f64,
    pub min_magnitude_2: f32,
}
