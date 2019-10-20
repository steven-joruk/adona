use chrono::{DateTime, Local};
use json;
use regex::Regex;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;

use crate::error::{Error, Result};
use crate::zip_archive;

pub trait AddonSource {
    fn fetch_details(&self) -> Result<FetchedDetails>;
    fn download_and_install<P: AsRef<Path>>(&self, from: &str, to: P) -> Result<()>;
}

#[derive(Clone, Debug, Deserialize, Eq)]
pub struct Addon {
    pub name: String,
    pub description: Option<String>,
    pub support_text: Option<String>,
    pub support_link: Option<String>,
    #[serde(rename = "source")]
    // TODO: Store a reference to an &AddonSource so that any can be used.
    source: GitHubRelease,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct GitHubRelease {
    owner: String,
    repo: String,
    regex: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FetchedDetails {
    pub address: String,
    pub released: DateTime<Local>,
    pub downloads: u32,
    pub version: String,
}

impl AddonSource for Addon {
    fn fetch_details(&self) -> Result<FetchedDetails> {
        let uri = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.source.owner, self.source.repo
        );

        let text = reqwest::get(&uri)?.text()?;
        let data = json::parse(&text)?;

        let version = data["tag_name"]
            .as_str()
            .ok_or(Error::ResponseMissingVersion)?
            .into();

        let released = data["published_at"]
            .as_str()
            .ok_or(Error::ResponseMissingReleaseDate)?
            .parse()?;

        let re = Regex::new(&self.source.regex)?;

        for asset in data["assets"].members() {
            let address: String = asset["browser_download_url"]
                .as_str()
                .ok_or(Error::ResponseMissingDownloadAddress)?
                .into();

            println!("Found address {}", &address);

            if re.is_match(&address) {
                let downloads = asset["download_count"]
                    .as_u32()
                    .ok_or(Error::ResponseMissingDownloadCount)?;

                return Ok(FetchedDetails {
                    address,
                    released,
                    downloads,
                    version,
                });
            }
        }

        Err(Error::AssetRegexDidNotMatch)
    }

    fn download_and_install<P: AsRef<Path>>(&self, from: &str, to: P) -> Result<()> {
        let mut buf = Vec::new();
        reqwest::get(from)?.read_to_end(&mut buf)?;
        zip_archive::extract(&buf, to)?;
        Ok(())
    }
}
