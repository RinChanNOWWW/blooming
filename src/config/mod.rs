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

use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use serfig::collectors::from_file;

use crate::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct QQBotConfig {
    /// The name of the bot.
    pub name: String,
    /// The uin (QQ number) of the bot.
    pub uin: String,
    /// go-cqhttp HTTP api. like: http://qqbot.me
    pub api: String,
    /// qq personal contacts
    pub dms: Vec<u64>,
    /// qq groups
    pub groups: Vec<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// config of qq bot.
    pub qq: QQBotConfig,
    /// mikan
    pub mikan: Option<MikanConfig>,
    /// byrbt
    pub byrbt: Option<ByrbtConfig>,
    /// tjupt
    pub tjupt: Option<TjuptConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
#[serde(default)]
pub struct ClapConfig {
    #[clap(long, short = 'c', required = true)]
    pub config_file: String,

    #[clap(long, short = 'd', action, default_value_t)]
    pub daemonize: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MikanConfig {
    // mikan rss link.
    pub rss: String,

    // Time interval for checking rss.
    pub interval: u64,

    // Proxy address
    pub proxy: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ByrbtConfig {
    // rss links
    pub rsses: Vec<String>,

    // Time interval for checking rss.
    pub interval: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TjuptConfig {
    // rss links
    pub rsses: Vec<String>,

    // Time interval for checking rss.
    pub interval: u64,
}

impl Config {
    pub fn load(file: &str) -> Result<Self> {
        let mut builder = serfig::Builder::default();
        builder = builder.collect(from_file(serfig::parsers::Toml, file));
        builder.build()
    }
}
