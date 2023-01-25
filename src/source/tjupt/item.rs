// Copyright 2023 waruto210.
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

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct TjuptRSSContent {
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    pub language: String,
    pub title: String,
    pub description: String,
    pub image: Image,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub generator: String,
    pub link: String,
    pub copyright: String,
    #[serde(rename = "item")]
    pub items: Vec<TjuptRSSItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub link: String,
    pub width: String,
    pub height: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TjuptRSSItem {
    pub title: String,
    pub description: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub link: String,
    pub guid: String,
    pub author: String,
    // TODO: parse comments when serde_xml_rs supports namespace.
    // pub comments: String,
    pub category: String,
}
