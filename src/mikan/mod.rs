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

use std::fmt::Display;

use chrono::DateTime;
use chrono::Local;

use self::rss::MikanRSSItem;

pub mod rss;

#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub pub_date: DateTime<Local>,
}

impl From<MikanRSSItem> for Item {
    fn from(item: MikanRSSItem) -> Self {
        let mut date = item.torrent.pub_date.clone();
        date.push_str("+08:00");
        let pub_date = DateTime::parse_from_rfc3339(&date).unwrap();
        let pub_date = pub_date.with_timezone(&Local {});
        Item {
            title: item.title,
            pub_date,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.pub_date)
    }
}

pub fn stringify_items(items: &[Item]) -> String {
    items
        .iter()
        .fold(String::from(""), |acc, item| format!("{}\n{}", acc, item))
}
