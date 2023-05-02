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

mod byrbt;
mod factory;
mod item;
mod mikan;
mod tjupt;

use std::sync::Arc;
use std::time::Duration;

pub use byrbt::ByrbtRSSContent;
pub use factory::register;
pub use factory::SourceFactory;
pub use item::Item;
pub use mikan::MikanRSSContent;
pub use tjupt::TjuptRSSContent;

use crate::Result;

#[async_trait::async_trait]
pub trait Source: Send + Sync {
    /// The name of the source. Eg. mikan, byrbt.
    fn name(&self) -> String;
    /// Pull items from the source.
    async fn pull_items(&self) -> Result<Vec<Item>>;
    /// The time interval between two pulls.
    fn interval(&self) -> Duration;
    /// Get RSS subscription links.
    fn rsses(&self) -> Vec<String>;
    /// Check connection to the RSS source.
    async fn check_connection(&self) -> Result<()> {
        let handles = self
            .rsses()
            .into_iter()
            .map(|rss| async move { reqwest::get(&rss).await })
            .collect::<Vec<_>>();

        futures::future::try_join_all(handles).await?;

        Ok(())
    }
}

pub type SourcePtr = Arc<dyn Source>;
