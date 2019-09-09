use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Deserialize, Eq, Serialize)]
pub struct Game {
    pub name: String,
    pub path: PathBuf,
    pub addons: HashSet<String>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for Game {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Settings {
    pub games: HashSet<Game>,
}

impl Settings {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Settings> {
        // TODO: If it doesn't exist, return defaults
        if !path.as_ref().exists() {
            return Ok(Settings::default());
        }

        println!("Opening {}", path.as_ref().display());
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let settings = serde_json::from_reader(reader)?;
        Ok(settings)
    }
}
