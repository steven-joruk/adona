use std::collections::HashSet;
use std::path::Path;
use std::{fs, io};

use crate::addon::Addon;
use crate::error::Result;
use serde_json;

pub struct Database;

impl Database {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<HashSet<Addon>> {
        println!("Opening {}", path.as_ref().display());
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let addons: HashSet<Addon> = serde_json::from_reader(reader)?;

        Ok(addons)
    }
}
