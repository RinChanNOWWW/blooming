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

#![feature(try_blocks)]

use std::env::current_dir;
use std::fs::File;
use std::path::PathBuf;

use backon::ConstantBuilder;
use backon::Retryable;
use blooming::notifier;
use blooming::source::register;
use blooming::source::SourceFactory;
use blooming::source::SourcePtr;
use blooming::ClapConfig;
use blooming::Config;
use blooming::Notifier;
use blooming::QQGuildNotifier;
use blooming::QQNotifier;
use blooming::Result;
use chrono::Local;
use clap::Parser;
use daemonize::Daemonize;
use log::error;
use log::info;
use reqwest::Client;
use tokio::task::JoinHandle;

const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn main_impl(config: Config) -> Result<()> {
    let mut factory = SourceFactory::default();
    register(&mut factory, &config)?;

    let client = Client::new();
    let mut handles = Vec::new();

    if let Some(qq) = config.qq.clone() {
        let notifier = notifier::QQNotifier::new(client.clone(), qq);
        handles.extend(activate_qq_notifier(&factory, notifier));
    }
    if let Some(qq_guild) = config.qq_guild.clone() {
        let notifier = notifier::QQGuildNotifier::new(client, qq_guild);
        handles.extend(activate_qq_guild_notifier(&factory, notifier));
    }

    futures::future::join_all(handles).await;

    Ok(())
}

fn activate_qq_notifier(factory: &SourceFactory, notifier: QQNotifier) -> Vec<JoinHandle<()>> {
    let sources = factory.sources();
    sources
        .iter()
        .map(|source| {
            let source = source.clone();
            let n = notifier.clone();
            tokio::spawn(async move {
                run(source, n).await;
            })
        })
        .collect::<Vec<_>>()
}

fn activate_qq_guild_notifier(
    factory: &SourceFactory,
    notifier: QQGuildNotifier,
) -> Vec<JoinHandle<()>> {
    let sources = factory.sources();
    sources
        .iter()
        .map(|source| {
            let source = source.clone();
            let n = notifier.clone();
            tokio::spawn(async move {
                run(source, n).await;
            })
        })
        .collect::<Vec<_>>()
}

async fn run<T: Notifier>(source: SourcePtr, mut notifier: T) {
    if source.check_connection().await.is_err() {
        error!("Check connection of '{}' failed", source.name());
    } else {
        info!("Check connection of '{}' successful", source.name());
    }

    let mut last_update = Local::now();
    let interval = source.interval();
    let retry_config = ConstantBuilder::default();
    let each_notify = notifier.num_items_each_notify();

    loop {
        tokio::time::sleep(interval).await;

        let result: Result<()> = try {
            let fetch = || async { source.pull_items().await };
            let items = fetch.retry(&retry_config).await?;
            let new_items = items
                .into_iter()
                .filter(|item| item.pub_date > last_update)
                .collect::<Vec<_>>();

            if !new_items.is_empty() {
                // update the time marker
                last_update = new_items.iter().fold(new_items[0].pub_date, |acc, item| {
                    let pub_time = item.pub_date;
                    if pub_time > acc { pub_time } else { acc }
                });

                // notify
                if each_notify == 0 {
                    notifier.notify(&source.name(), new_items.clone()).await?;
                } else {
                    for chunk in new_items.chunks(each_notify) {
                        notifier.notify(&source.name(), chunk.to_vec()).await?;
                    }
                }
            }
        };

        if let Err(e) = result {
            error!("{}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_local!();

    let args = ClapConfig::parse();

    let config = Config::load(&args.config_file)?;
    info!("Welcome to use blooming (version: {})", VERSION);
    info!("Starting blooming with config: {:?}", config);

    if args.daemonize {
        let current_dir = current_dir()?;
        let log_file = PathBuf::from(format!("{}/blooming.log", current_dir.display()));
        let pid_file = PathBuf::from(format!("{}/blooming.pid", current_dir.display()));

        let stdout = File::create(log_file.clone())?;
        let stderr = File::create(log_file)?;
        let daemon = Daemonize::new()
            .pid_file(pid_file)
            .working_directory(current_dir)
            .stdout(stdout)
            .stderr(stderr);

        daemon.start()?;
    }

    main_impl(config).await
}
