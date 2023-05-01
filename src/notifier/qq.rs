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

use log::error;
use log::info;
use reqwest::Client;
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
    name: String,
    uin: String,
    api: String,
    dms: Vec<u64>,
    groups: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct PrivateMsg {
    user_id: u64,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct GroupMsg {
    group_id: u64,
    messages: Vec<Message>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Message {
    #[serde(rename = "type")]
    msg_type: String,
    data: Data,
}

#[derive(Clone, Serialize, Deserialize)]
struct Data {
    #[serde(rename = "name")]
    sender_name: String,
    #[serde(rename = "uin")]
    sender_uin: String,
    content: String,
}

impl Notifier {
    pub fn new(name: String, uin: String, api: String, dms: Vec<u64>, groups: Vec<u64>) -> Self {
        Self {
            name,
            uin,
            client: Client::new(),
            api,
            dms,
            groups,
        }
    }
}

impl QQNotifier {
    pub fn new(name: String, uin: String, api: String, dms: Vec<u64>, groups: Vec<u64>) -> Self {
        Self {
            inner: Arc::new(Notifier::new(name, uin, api, dms, groups)),
        }
    }

    pub async fn notify(&self, source: &str, items: Vec<Item>) -> Result<()> {
        let mut pm_handles = Vec::new();

        {
            let notifier = self.inner.clone();
            let pm_items = items.clone();
            let source = source.to_string();
            let dms_handle = async move {
                let mut msgs = Vec::with_capacity(pm_items.len() * 2);
                for item in pm_items.iter() {
                    msgs.extend(Self::wrap_item(&notifier, &source, item));
                }
                if let Err(e) = Self::send_private_msg(&notifier, msgs).await {
                    error!("Send private msg failed: {}", e);
                }
            };
            pm_handles.push(dms_handle);
        }

        let mut dm_handles = Vec::new();

        {
            let notifier = self.inner.clone();
            let gp_items = items;
            let source = source.to_string();
            let groups_handle = async move {
                let mut msgs = Vec::with_capacity(gp_items.len() * 2);
                for item in gp_items.iter() {
                    msgs.extend(Self::wrap_item(&notifier, &source, item));
                }
                if let Err(e) = Self::send_group_msg(&notifier, msgs).await {
                    error!("Send group msg failed: {}", e);
                }
            };
            dm_handles.push(groups_handle);
        }

        let join_dm = futures::future::join_all(dm_handles);
        let join_pm = futures::future::join_all(pm_handles);

        monoio::join!(join_dm, join_pm);

        Ok(())
    }

    fn wrap_item(notifier: &Notifier, source: &str, item: &Item) -> Vec<Message> {
        vec![
            Message {
                msg_type: "node".to_string(),
                data: Data {
                    sender_name: notifier.name.clone(),
                    sender_uin: notifier.uin.clone(),
                    content: format!("{}:\n{} ({})", source, item.title, item.pub_date),
                },
            },
            Message {
                msg_type: "node".to_string(),
                data: Data {
                    sender_name: notifier.name.clone(),
                    sender_uin: notifier.uin.clone(),
                    content: item.url.clone(),
                },
            },
        ]
    }

    async fn send_private_msg(notifier: &Notifier, msg: Vec<Message>) -> Result<()> {
        let url = format!("{}/send_private_forward_msg", notifier.api);

        for user_id in notifier.dms.iter() {
            let body = PrivateMsg {
                user_id: *user_id,
                messages: msg.clone(),
            };
            notifier.client.post(url.clone()).json(&body).send().await?;
            info!("Notified user {}", user_id);
        }
        Ok(())
    }

    async fn send_group_msg(notifier: &Notifier, msg: Vec<Message>) -> Result<()> {
        let url = format!("{}/send_group_forward_msg", notifier.api);
        for group_id in notifier.groups.iter() {
            let body = GroupMsg {
                group_id: *group_id,
                messages: msg.clone(),
            };
            notifier.client.post(url.clone()).json(&body).send().await?;
            info!("Notified group {}", group_id);
        }
        Ok(())
    }
}
