// Copyright 2023 RinChanNOWWW
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

use yaserde_derive::YaDeserialize;
use yaserde_derive::YaSerialize;

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct ByrbtRSSContent {
    pub channel: Channel,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: String,
    pub copyright: String,
    #[yaserde(rename = "managingEditor")]
    pub managing_editor: String,
    #[yaserde(rename = "webMaster")]
    pub web_master: String,
    #[yaserde(rename = "pubDate")]
    pub pub_date: String,
    pub generator: String,
    pub docs: String,
    #[yaserde(rename = "item")]
    pub items: Vec<ByrbtRSSItem>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub link: String,
    pub width: String,
    pub height: String,
    pub description: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct ByrbtRSSItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub author: String,
    pub category: String,
    pub comments: String,
    pub guid: String,
    #[yaserde(rename = "pubDate")]
    pub pub_date: String,
}
