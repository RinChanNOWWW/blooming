use log::error;
use log::info;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::mikan::Item;
use crate::Result;

#[derive(Clone)]
pub struct QQNotifer {
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

impl QQNotifer {
    pub fn new(api: String, dms: Vec<u64>, groups: Vec<u64>) -> QQNotifer {
        Self {
            client: Client::new(),
            api,
            dms,
            groups,
        }
    }

    pub fn notify(&self, items: &[Item]) -> Result<()> {
        let items1 = items.to_vec();
        let items2 = items1.clone();

        let private_notifier = self.clone();
        let group_notifier = private_notifier.clone();

        let dms_handle = std::thread::spawn(move || {
            for item in items1.iter() {
                let msg = format!("NEWLY UPDATE:\n{}", item);
                if let Err(e) = private_notifier.send_private_msg(msg) {
                    error!("Send private msg failed: {}", e);
                }
            }
        });
        let groups_handle = std::thread::spawn(move || {
            for item in items2.iter() {
                let msg = format!("NEWLY UPDATE:\n{}", item);
                if let Err(e) = group_notifier.send_group_msg(msg) {
                    error!("Send private msg failed: {}", e);
                }
            }
        });

        dms_handle.join().unwrap();
        groups_handle.join().unwrap();
        Ok(())
    }

    fn send_private_msg(&self, msg: String) -> Result<()> {
        let url = format!("{}/send_private_msg", self.api);

        for user_id in self.dms.iter() {
            let body = PrivateMsg {
                user_id: *user_id,
                message: msg.clone(),
            };
            self.client.post(url.clone()).json(&body).send()?;
            info!("Notified user {}", user_id);
        }
        Ok(())
    }

    fn send_group_msg(&self, msg: String) -> Result<()> {
        let url = format!("{}/send_group_msg", self.api);
        for group_id in self.groups.iter() {
            let body = GroupMsg {
                group_id: *group_id,
                message: msg.clone(),
            };
            self.client.post(url.clone()).json(&body).send()?;
            info!("Notified group {}", group_id);
        }
        Ok(())
    }
}
