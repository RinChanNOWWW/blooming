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

use std::time::Duration;

use log::error;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::source::Item;
use crate::Notifier;
use crate::QQBotConfig;
use crate::Result;

#[derive(Clone)]
pub struct CQHTTPNotifier {
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

#[async_trait::async_trait]
impl Notifier for CQHTTPNotifier {
    async fn notify(&mut self, source: &str, items: Vec<Item>) -> Result<()> {
        let delay = self.conf.delay;
        let pm_handle = {
            let client = self.client.clone();
            let pm_items = items.clone();
            let source = source.to_string();
            let mut msgs = Vec::with_capacity(pm_items.len() * 2);
            for item in pm_items.iter() {
                msgs.extend(self.messages(&source, item));
            }
            let msgs = self.private_messages(msgs);
            let url = format!("{}/send_private_forward_msg", self.conf.api);
            async move {
                if let Err(e) = Self::send_messages(client, &url, msgs, delay).await {
                    error!("Send private msg failed: {}", e);
                }
            }
        };

        let dm_handle = {
            let client = self.client.clone();
            let gp_items = items;
            let source = source.to_string();
            let mut msgs = Vec::with_capacity(gp_items.len() * 2);
            for item in gp_items.iter() {
                msgs.extend(self.messages(&source, item));
            }
            let msgs = self.group_messages(msgs);
            let url = format!("{}/send_group_forward_msg", self.conf.api);
            async move {
                if let Err(e) = Self::send_messages(client, &url, msgs, delay).await {
                    error!("Send group msg failed: {}", e);
                }
            }
        };

        tokio::join!(pm_handle, dm_handle);

        Ok(())
    }
}

impl CQHTTPNotifier {
    pub fn new(client: Client, conf: QQBotConfig) -> Self {
        Self { client, conf }
    }

    fn messages(&self, source: &str, item: &Item) -> Vec<Message> {
        let mut messages = vec![Message {
            msg_type: "node".to_string(),
            data: Data {
                sender_name: self.conf.name.clone(),
                sender_uin: self.conf.uin.clone(),
                content: format!("{}:\n{} ({})", source, item.title, item.pub_date),
            },
        }];
        if self.conf.with_torrent {
            messages.push(Message {
                msg_type: "node".to_string(),
                data: Data {
                    sender_name: self.conf.name.clone(),
                    sender_uin: self.conf.uin.clone(),
                    content: item.url.clone(),
                },
            });
        }
        messages
    }

    fn private_messages(&self, msg: Vec<Message>) -> Vec<PrivateMsg> {
        let mut msgs = Vec::with_capacity(self.conf.dms.len());
        for user_id in self.conf.dms.iter() {
            msgs.push(PrivateMsg {
                user_id: *user_id,
                messages: msg.clone(),
            });
        }
        msgs
    }

    fn group_messages(&self, msg: Vec<Message>) -> Vec<GroupMsg> {
        let mut msgs = Vec::with_capacity(self.conf.groups.len());
        for group_id in self.conf.groups.iter() {
            msgs.push(GroupMsg {
                group_id: *group_id,
                messages: msg.clone(),
            });
        }
        msgs
    }

    async fn send_messages<T: Serialize>(
        client: Client,
        url: &str,
        msgs: Vec<T>,
        delay: u64,
    ) -> Result<()> {
        for msg in msgs.iter() {
            client.post(url).json(msg).send().await?;
            tokio::time::sleep(Duration::from_micros(delay)).await;
        }
        Ok(())
    }
}
