// Copyright 2022 RinChanNOWWW.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
