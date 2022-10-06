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

use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use serfig::collectors::from_file;
use serfig::collectors::from_self;

use crate::Result;

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
#[serde(default)]
pub struct QQBotConfig {
    // go-cqhttp HTTP api. like: http://qqbot.me
    #[clap(long, default_value_t)]
    pub api: String,

    // qq personal contacts
    #[clap(long)]
    pub dms: Vec<u64>,

    // qq groups
    #[clap(long)]
    pub groups: Vec<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
#[serde(default)]
pub struct Config {
    #[clap(long, short = 'c', default_value = "config.toml")]
    pub config_file: String,

    #[clap(long, short = 'd', action, default_value_t)]
    pub daemonize: bool,

    // mikan rss link.
    #[clap(long, default_value_t)]
    pub rss: String,

    // Time interval for checking rss.
    #[clap(long, default_value_t)]
    pub interval: u64,

    // config of qq bot.
    #[clap(flatten)]
    pub qq: QQBotConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let arg_conf = Self::parse();
        let mut builder = serfig::Builder::default();

        builder = builder.collect(from_file(serfig::parsers::Toml, &arg_conf.config_file));

        builder = builder.collect(from_self(arg_conf));

        builder.build()
    }
}
