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

use std::sync::Arc;
use std::time::Duration;

use anyhow::Error;

use super::item::TjuptRSSContent;
use crate::source::Item;
use crate::source::Source;
use crate::source::SourcePtr;
use crate::Result;
use crate::TjuptConfig;

pub struct TjuptSource {
    rsses: Vec<String>,
    interval: Duration,
}

impl TjuptSource {
    pub fn create(config: &TjuptConfig) -> SourcePtr {
        Arc::new(Self {
            rsses: config.rsses.clone(),
            interval: Duration::from_secs(config.interval),
        })
    }
}

impl Source for TjuptSource {
    fn name(&self) -> String {
        "TJUPT".to_string()
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    fn pull_items(&self) -> Result<Vec<Item>> {
        let contents = self
            .rsses
            .iter()
            .map(|rss| {
                let resp = reqwest::blocking::get(rss)?;
                let content = resp.text()?;
                let content: TjuptRSSContent =
                    yaserde::de::from_str(&content).map_err(Error::msg)?;
                Ok(content)
            })
            .collect::<Result<Vec<_>>>()?;

        let items = contents
            .into_iter()
            .flat_map(|content| content.channel.items)
            .map(Item::from)
            .collect::<Vec<_>>();

        Ok(items)
    }
}
