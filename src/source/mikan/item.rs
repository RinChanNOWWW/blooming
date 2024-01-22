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

use std::io::BufRead;

use chrono::DateTime;
use chrono::Local;

use crate::Item;
use crate::Result;

pub struct Mikan;

impl Mikan {
    pub fn parse_items<R: BufRead>(content: R) -> Result<Vec<Item>> {
        let channel = rss_for_mikan::Channel::read_from(content)?;

        Ok(channel
            .items
            .into_iter()
            .map(|item| {
                let mut date = item.torrent.unwrap().pub_date.unwrap();
                date.push_str("+08:00");
                let pub_date = DateTime::parse_from_rfc3339(&date)
                    .unwrap()
                    .with_timezone(&Local {});
                Item {
                    title: item.title.unwrap(),
                    pub_date,
                    url: item.enclosure.unwrap().url,
                }
            })
            .collect::<Vec<_>>())
    }
}
