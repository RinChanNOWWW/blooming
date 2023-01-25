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

use log::error;
use log::info;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::source::Item;
use crate::Result;

#[derive(Clone)]
pub struct QQNotifier {
    inner: Arc<Notifier>,
}

struct Notifier {
    client: Client,
    api: String,
    dms: Vec<u64>,
    groups: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct PrivateMsg {
    user_id: u64,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct GroupMsg {
    group_id: u64,
    message: String,
}

impl Notifier {
    pub fn new(api: String, dms: Vec<u64>, groups: Vec<u64>) -> Self {
        Self {
            client: Client::new(),
            api,
            dms,
            groups,
        }
    }
}

impl QQNotifier {
    pub fn new(api: String, dms: Vec<u64>, groups: Vec<u64>) -> Self {
        Self {
            inner: Arc::new(Notifier::new(api, dms, groups)),
        }
    }

    pub fn notify(&self, source: &str, items: Vec<Item>) -> Result<()> {
        let mut handles = Vec::new();

        {
            let notifier = self.inner.clone();
            let pm_items = items.clone();
            let source = source.to_string();
            let dms_handle = std::thread::spawn(move || {
                for item in pm_items.iter() {
                    let msg = format!("{}:\n{} ({})", source, item.title, item.pub_date);
                    if let Err(e) = Self::send_private_msg(notifier.clone(), msg) {
                        error!("Send private msg failed: {}", e);
                    }
                }
            });
            handles.push(dms_handle);
        }

        {
            let notifier = self.inner.clone();
            let gp_items = items;
            let source = source.to_string();
            let groups_handle = std::thread::spawn(move || {
                for item in gp_items.iter() {
                    let msg = format!("{}:\n{} ({})", source, item.title, item.pub_date);
                    if let Err(e) = Self::send_group_msg(notifier.clone(), msg) {
                        error!("Send group msg failed: {}", e);
                    }
                }
            });
            handles.push(groups_handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    fn send_private_msg(notifier: Arc<Notifier>, msg: String) -> Result<()> {
        let url = format!("{}/send_private_msg", notifier.api);

        for user_id in notifier.dms.iter() {
            let body = PrivateMsg {
                user_id: *user_id,
                message: msg.clone(),
            };
            notifier.client.post(url.clone()).json(&body).send()?;
            info!("Notified user {}", user_id);
        }
        Ok(())
    }

    fn send_group_msg(notifier: Arc<Notifier>, msg: String) -> Result<()> {
        let url = format!("{}/send_group_msg", notifier.api);
        for group_id in notifier.groups.iter() {
            let body = GroupMsg {
                group_id: *group_id,
                message: msg.clone(),
            };
            notifier.client.post(url.clone()).json(&body).send()?;
            info!("Notified group {}", group_id);
        }
        Ok(())
    }
}
