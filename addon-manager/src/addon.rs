use async_trait::async_trait;
use chrono::{DateTime, Local};
use json;
use regex::Regex;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::path::Path;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::error::{Error, Result};

#[async_trait]
pub trait AddonSource {
    /// Gather any relevant details from the GitHub API.
    async fn fetch_details(&mut self) -> Result<()>;

    /// Returns the download address of the addon zip file, if it is known.
    fn download_address(&self) -> Option<&str>;

    /// Attempts to download the addon zip file.
    ///
    /// You will have to `fetch_details` in order for `download_address` to
    /// return a `Some` value.
    async fn download<W>(&self, to: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin + ?Sized,
    {
        let from = self.download_address().expect("TODO: error from None");
        tokio::io::copy(reqwest::get(from).await?.bytes_stream().await?.into(), to).await?;
        Ok(())
    }

    /// From must be an addon zip file.
    async fn install<B, P>(from: B, to: P) -> Result<()>
    where
        B: AsRef<[u8]>,
        P: AsRef<Path>,
    {
        let cursor = std::io::Cursor::new(from);
        let mut archive = zip::ZipArchive::new(cursor)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let path = file.sanitized_name();
            if file.name().ends_with('/') {
                std::fs::create_dir_all(to.as_ref().join(path))?;
            } else {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(to.as_ref().join(parent))?;
                }

                let mut out = std::fs::File::create(to.as_ref().join(path))?;
                tokio::io::copy(&mut file, &mut out).await?;
            }
        }

        Ok(())
    }

}

#[derive(Clone, Debug, Deserialize, Eq)]
pub struct Addon {
    pub name: String,
    pub description: Option<String>,
    pub support_text: Option<String>,
    pub support_link: Option<String>,

    owner: String,
    repo: String,
    regex: String,

    #[serde(skip)]
    download_address: Option<String>,

    #[serde(skip)]
    release_date: Option<DateTime<Local>>,

    #[serde(skip)]
    download_count: Option<u32>,

    #[serde(skip)]
    version: Option<String>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FetchedDetails {}

#[async_trait]
impl AddonSource for Addon {
    async fn fetch_details(&mut self) -> Result<()> {
        let uri = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.owner, self.repo
        );

        let text = reqwest::get(&uri).await?.text().await?;
        let data = json::parse(&text)?;

        self.version = Some(
            data["tag_name"]
                .as_str()
                .ok_or(Error::ResponseMissingVersion)?
                .into(),
        );

        self.release_date = Some(
            data["published_at"]
                .as_str()
                .ok_or(Error::ResponseMissingReleaseDate)?
                .parse()?,
        );

        let re = Regex::new(&self.regex)?;

        for asset in data["assets"].members() {
            let download_address = asset["browser_download_url"]
                .as_str()
                .ok_or(Error::ResponseMissingDownloadAddress)?.to_string();

            if re.is_match(&download_address) {
                self.download_address = Some(download_address);
                self.download_count = Some(
                    asset["download_count"]
                        .as_u32()
                        .ok_or(Error::ResponseMissingDownloadCount)?,
                );
                return Ok(());
            }
        }

        Err(Error::AssetRegexDidNotMatch)
    }

    fn download_address(&self) -> Option<&str> {
        &self.download_address
    }
}
