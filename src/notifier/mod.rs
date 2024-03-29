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

mod qq;
mod qq_guild;

pub use qq::QQNotifier;
pub use qq_guild::QQGuildNotifier;

use crate::source::Item;
use crate::Result;

#[async_trait::async_trait]
pub trait Notifier: Sync + Send + Clone {
    async fn notify(&mut self, source: &str, items: Vec<Item>) -> Result<()>;

    /// The number of items to be notified each time.
    ///
    /// If it is 0, all items will be notified at once.
    fn num_items_each_notify(&self) -> usize {
        0
    }
}
