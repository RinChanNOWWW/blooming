use reqwest;
use serde::Deserialize;
use serde::Serialize;

use crate::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct MikanRSSContent {
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    #[serde(rename = "item")]
    pub items: Vec<MikanRSSItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MikanRSSItem {
    pub guid: String,
    pub link: String,
    pub title: String,
    pub description: String,
    pub torrent: Torrent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub link: String,
    #[serde(rename = "contentLength")]
    pub content_length: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
}

pub fn get_rss_content(url: &str) -> Result<MikanRSSContent> {
    let resp = reqwest::blocking::get(url)?;
    let rss = resp.text()?;
    let rss = serde_xml_rs::from_str(&rss)?;
    Ok(rss)
}
