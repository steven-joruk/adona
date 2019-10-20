use std::collections::HashSet;
use std::path::Path;
use std::{fs, io};

use crate::addon::Addon;
use crate::error::Result;
use serde_json;

const DEFAULT_DATA: &str = include_str!("../res/wow-classic.json");

pub struct Database {
    pub addons: HashSet<Addon>,
}

impl Database {
    pub fn update(&self) {
        println!("TODO: Update");
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Database> {
        let addons: HashSet<Addon>;

        match fs::File::open(path) {
            Ok(file) => {
                println!("Loading the database from {}", path.as_ref().display());
                let reader = io::BufReader::new(file);
                addons = serde_json::from_reader(reader)?;
            }
            Err(e) => {
                eprintln!("Unable to open {}: {}", path.as_ref().display(), e);
                println!("Loading the built in database");
                addons = serde_json::from_str(DEFAULT_DATA)?;
            }
        };

        Ok(Database { addons })
    }
}
