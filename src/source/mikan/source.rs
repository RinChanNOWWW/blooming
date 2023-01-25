// Copyright 2023 RinChanNOWWW.
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

use std::sync::Arc;
use std::time::Duration;

use super::item::MikanRSSContent;
use crate::source::Item;
use crate::source::Source;
use crate::source::SourcePtr;
use crate::MikanConfig;
use crate::Result;

pub struct MikanSource {
    rss: String,
    interval: Duration,
}

impl MikanSource {
    pub fn create(config: &MikanConfig) -> SourcePtr {
        Arc::new(Self {
            rss: config.rss.clone(),
            interval: Duration::from_secs(config.interval),
        })
    }
}

impl Source for MikanSource {
    fn name(&self) -> String {
        "Mikan".to_string()
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    fn pull_items(&self) -> Result<Vec<Item>> {
        let resp = reqwest::blocking::get(&self.rss)?;
        let content = resp.text()?;
        let content: MikanRSSContent = serde_xml_rs::from_str(&content)?;

        Ok(content
            .channel
            .items
            .into_iter()
            .map(Item::from)
            .collect::<Vec<_>>())
    }
}
