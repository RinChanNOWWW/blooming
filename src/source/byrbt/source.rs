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

use super::item::ByrbtRSSContent;
use crate::source::Item;
use crate::source::Source;
use crate::source::SourcePtr;
use crate::ByrbtConfig;
use crate::Result;

pub struct ByrbtSource {
    rsses: Vec<String>,
    interval: Duration,
}

impl ByrbtSource {
    pub fn create(config: &ByrbtConfig) -> SourcePtr {
        Arc::new(Self {
            rsses: config.rsses.clone(),
            interval: Duration::from_secs(config.interval),
        })
    }
}

#[async_trait::async_trait]
impl Source for ByrbtSource {
    fn name(&self) -> String {
        "BYRBT".to_string()
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    async fn pull_items(&self) -> Result<Vec<Item>> {
        let handles = self
            .rsses
            .iter()
            .map(|rss| {
                let rss = rss.clone();
                async move {
                    let content = reqwest::get(rss).await?.text().await?;
                    yaserde::de::from_str::<ByrbtRSSContent>(&content).map_err(Error::msg)
                }
            })
            .collect::<Vec<_>>();

        let contents = futures::future::try_join_all(handles).await?;

        let items = contents
            .into_iter()
            .flat_map(|content| content.channel.items)
            .map(Item::from)
            .collect::<Vec<_>>();

        Ok(items)
    }

    fn rsses(&self) -> Vec<String> {
        self.rsses.clone()
    }
}
