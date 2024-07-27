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

use log::info;
use reqwest::Client;
use reqwest::Response;
use serde::Deserialize;
use serde::Serialize;

use crate::source::Item;
use crate::Notifier;
use crate::QQBotConfig;
use crate::Result;

const API_GET_ACCESS_TOKEN: &str = "https://bots.qq.com/app/getAppAccessToken";
const API_BOT: &str = "https://api.sgroup.qq.com";
const API_BOT_SANDBOX: &str = "https://sandbox.api.sgroup.qq.com";

const CODE_TOKEN_EXPIRED: i32 = 11244;

#[derive(Clone)]
pub struct QQNotifier {
    client: Client,
    conf: QQBotConfig,

    api: String,
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct GetAppAccessTokenReq {
    #[serde(rename = "appId")]
    app_id: String,
    #[serde(rename = "clientSecret")]
    app_secret: String,
}

#[derive(Serialize, Deserialize)]
struct GetAppAccessTokenResp {
    access_token: String,
    expires_in: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorMessage {
    message: String,
    code: i32,
    err_code: i32,
    trace_id: String,
}

/// Check the response status code and return whether the token is valid.
fn check_response(resp: &Response) -> Result<bool> {
    let status_code = resp.status().as_u16();
    if status_code == 401 {
        return Ok(false);
    }
    if status_code == 500 {
        let err_msg = resp.json::<ErrorMessage>().await?;
        if err_msg.code == CODE_TOKEN_EXPIRED {
            return Ok(false);
        } else {
            return Err(anyhow::anyhow!(err_msg.message));
        }
    }

    Ok(true)
}

#[async_trait::async_trait]
impl Notifier for QQNotifier {
    async fn notify(&mut self, source: &str, items: Vec<Item>) -> Result<()> {
        if self.access_token.is_empty() {
            self.access_token = self.get_access_token().await?;
        }

        let msg = self.message(source, items);

        let resp = self.send_message(&msg).await?;

        if check_response(&resp)? {
            Ok(())
        } else {
            info!("Token invalid, need to renew it.");
            self.access_token = self.get_access_token().await?;
            self.send_message(&msg).await
        }
    }

    fn num_items_each_notify(&self) -> usize {
        3
    }
}

impl QQNotifier {
    pub fn new(client: Client, conf: QQBotConfig) -> Self {
        let api = if conf.sandbox {
            format!("{}/channels/{}/messages", API_BOT_SANDBOX, conf.channel_id)
        } else {
            format!("{}/channels/{}/messages", API_BOT, conf.channel_id)
        };

        Self {
            client,
            conf,
            api,
            access_token: "".to_string(),
        }
    }

    async fn get_access_token(&self) -> Result<String> {
        let body = GetAppAccessTokenReq {
            app_id: self.conf.app_id.clone(),
            app_secret: self.conf.app_secret.clone(),
        };
        let resp = self
            .client
            .post(API_GET_ACCESS_TOKEN)
            .json(&body)
            .send()
            .await?;
        let resp = resp.json::<GetAppAccessTokenResp>().await?;
        Ok(resp.access_token)
    }

    fn message(&self, source: &str, items: Vec<Item>) -> Message {
        let mut msg = format!("{source}:\n");
        for item in items {
            msg.push_str(&format!("{} ({})\n", item.title, item.pub_date));
        }
        Message { content: msg }
    }

    async fn send_message(&self, msg: &Message) -> Result<Response> {
        let resp = self
            .client
            .post(&self.api)
            .header("Authorization", format!("QQBot {}", self.access_token))
            .header("X-Union-Appid", &self.conf.app_id)
            .json(msg)
            .send()
            .await?;
        Ok(resp)
    }
}
