use chrono::{DateTime, Local};
use json;
use reqwest;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

use crate::error::{Error, Result};

pub trait AddonSource {
    fn check(&self) -> Result<DateTime<Local>>;
    fn download(&self) -> Result<()>;
}

#[derive(Deserialize, Eq)]
pub struct Addon {
    pub name: String,
    pub stars: u32,
    pub description: String,
    pub released: DateTime<Local>,
    pub version: String,
    pub support_link: String,
    #[serde(rename = "source")]
    // TODO: Store a reference to an &AddonSource so that any can be used.
    pub source: GitHubRelease,
}

impl Hash for Addon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Addon {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Deserialize, Eq, PartialEq)]
pub struct GitHubRelease {
    owner: String,
    repo: String,
}

impl AddonSource for GitHubRelease {
    fn check(&self) -> Result<DateTime<Local>> {
        let uri = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.owner, self.repo
        );
        let text = reqwest::get(&uri)?.text()?;
        let data = json::parse(&text)?;

        if !data.has_key("published_at") {
            return Err(Error::GitHubResponse);
        }

        Ok(data["published_at"].as_str().unwrap().parse().unwrap())
    }

    fn download(&self) -> Result<()> {
        Ok(())
    }
}
