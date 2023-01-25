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
use std::sync::Arc;

use bloom::notifier;
use bloom::source::register;
use bloom::source::SourceFactory;
use bloom::source::SourcePtr;
use bloom::ClapConfig;
use bloom::Config;
use bloom::QQNotifier;
use bloom::Result;
use chrono::Local;
use clap::Parser;
use daemonize::Daemonize;
use log::error;
use log::info;

fn main_impl(config: Config) -> Result<()> {
    let qq_conf = &config.qq;
    let notifier = notifier::QQNotifier::new(
        qq_conf.api.clone(),
        qq_conf.dms.clone(),
        qq_conf.groups.clone(),
    );

    let mut factory = SourceFactory::default();
    register(&mut factory, &config);

    activate_sources(factory, Arc::new(notifier))
}

fn activate_sources(factory: SourceFactory, notifier: Arc<QQNotifier>) -> Result<()> {
    let sources = factory.sources();
    let handles = sources
        .iter()
        .map(|source| {
            let source = source.clone();
            let n = notifier.clone();
            std::thread::spawn(move || run(source, n))
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

fn run(source: SourcePtr, notifier: Arc<QQNotifier>) {
    let mut last_update = Local::now();
    let interval = source.interval();

    loop {
        std::thread::sleep(interval);

        let result: Result<()> = try {
            let items = source.pull_items()?;
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

                // notify by qq bot
                notifier.notify(&source.name(), new_items)?;
            }
        };

        if let Err(e) = result {
            error!("{}", e);
        }
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = ClapConfig::parse();

    let config = Config::load(&args.config_file)?;
    info!("Starting Mikan Notifier with config: {:?}", config);

    if args.daemonize {
        let current_dir = current_dir()?;
        let log_file = PathBuf::from(format!("{}/bloom.log", current_dir.display()));
        let pid_file = PathBuf::from(format!("{}/bloom.pid", current_dir.display()));

        let stdout = File::create(log_file.clone())?;
        let stderr = File::create(log_file)?;
        let daemon = Daemonize::new()
            .pid_file(pid_file)
            .working_directory(current_dir)
            .stdout(stdout)
            .stderr(stderr);

        daemon.start()?;
    }

    main_impl(config)
}
