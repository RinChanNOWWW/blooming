#![feature(try_blocks)]

use std::time;

use chrono::Local;
use log::error;
use log::info;
use mikan_notifier::mikan;
use mikan_notifier::mikan::stringify_items;
use mikan_notifier::mikan::Item;
use mikan_notifier::notifier;
use mikan_notifier::Config;
use mikan_notifier::Result;

fn get_new_items(rss: &str) -> Result<Vec<Item>> {
    let rss_content = mikan::rss::get_rss_content(rss)?;

    Ok(rss_content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>())
}

fn run(config: Config) -> Result<()> {
    let qq_conf = &config.qq;
    let notifer = notifier::QQNotifer::new(
        qq_conf.api.clone(),
        qq_conf.dms.clone(),
        qq_conf.groups.clone(),
    );

    // Firstly, intialize the global state
    let items = get_new_items(&config.rss)?;
    info!("Current items:\n{}", stringify_items(&items));

    let mut last_update = Local::now();

    loop {
        std::thread::sleep(time::Duration::from_secs(config.interval));

        let result: Result<()> = try {
            let items = get_new_items(&config.rss)?;
            notifer.notify(&items)?;
            let new_items = items
                .into_iter()
                .filter(|item| item.pub_date > last_update)
                .collect::<Vec<_>>();

            if !new_items.is_empty() {
                info!("New items:\n{}", stringify_items(&new_items));
                // notify by qq bot
                notifer.notify(&new_items)?;

                // update the time marker
                last_update = new_items.iter().fold(new_items[0].pub_date, |acc, item| {
                    if item.pub_date > acc {
                        item.pub_date
                    } else {
                        acc
                    }
                });
            }
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let config = Config::load()?;
    info!("Starting Mikan Notifier with config: {:?}", config);
    run(config)
}
