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

use log::error;
use log::info;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::source::Item;
use crate::QQBotConfig;
use crate::Result;

#[derive(Clone)]
pub struct QQNotifier {
    inner: Arc<Notifier>,
}

struct Notifier {
    client: Client,
    conf: QQBotConfig,
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
    pub fn new(conf: QQBotConfig) -> Self {
        Self {
            client: Client::new(),
            conf,
        }
    }
}

impl QQNotifier {
    pub fn new(conf: QQBotConfig) -> Self {
        Self {
            inner: Arc::new(Notifier::new(conf)),
        }
    }

    pub async fn notify(&self, source: &str, items: Vec<Item>) -> Result<()> {
        let pm_handle = {
            let notifier = self.inner.clone();
            let pm_items = items.clone();
            let source = source.to_string();
            async move {
                let mut msgs = Vec::with_capacity(pm_items.len() * 2);
                for item in pm_items.iter() {
                    msgs.extend(Self::wrap_item(&notifier, &source, item));
                }
                if let Err(e) = Self::send_private_msg(&notifier, msgs).await {
                    error!("Send private msg failed: {}", e);
                }
            }
        };

        let dm_handle = {
            let notifier = self.inner.clone();
            let gp_items = items;
            let source = source.to_string();
            async move {
                let mut msgs = Vec::with_capacity(gp_items.len() * 2);
                for item in gp_items.iter() {
                    msgs.extend(Self::wrap_item(&notifier, &source, item));
                }
                if let Err(e) = Self::send_group_msg(&notifier, msgs).await {
                    error!("Send group msg failed: {}", e);
                }
            }
        };

        tokio::join!(pm_handle, dm_handle);

        Ok(())
    }

    fn wrap_item(notifier: &Notifier, source: &str, item: &Item) -> Vec<Message> {
        let mut messages = vec![Message {
            msg_type: "node".to_string(),
            data: Data {
                sender_name: notifier.conf.name.clone(),
                sender_uin: notifier.conf.uin.clone(),
                content: format!("{}:\n{} ({})", source, item.title, item.pub_date),
            },
        }];
        if notifier.conf.with_torrent {
            messages.push(Message {
                msg_type: "node".to_string(),
                data: Data {
                    sender_name: notifier.conf.name.clone(),
                    sender_uin: notifier.conf.uin.clone(),
                    content: item.url.clone(),
                },
            });
        }
        messages
    }

    async fn send_private_msg(notifier: &Notifier, msg: Vec<Message>) -> Result<()> {
        let url = format!("{}/send_private_forward_msg", notifier.conf.api);

        for user_id in notifier.conf.dms.iter() {
            let body = PrivateMsg {
                user_id: *user_id,
                messages: msg.clone(),
            };
            notifier.client.post(url.clone()).json(&body).send().await?;
            info!("Notified user {}", user_id);
            tokio::time::sleep(Duration::from_micros(notifier.conf.delay)).await;
        }
        Ok(())
    }

    async fn send_group_msg(notifier: &Notifier, msg: Vec<Message>) -> Result<()> {
        let url = format!("{}/send_group_forward_msg", notifier.conf.api);
        for group_id in notifier.conf.groups.iter() {
            let body = GroupMsg {
                group_id: *group_id,
                messages: msg.clone(),
            };
            notifier.client.post(url.clone()).json(&body).send().await?;
            info!("Notified group {}", group_id);
            tokio::time::sleep(Duration::from_micros(notifier.conf.delay)).await;
        }
        Ok(())
    }
}
