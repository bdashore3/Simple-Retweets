mod helpers;
mod structures;

use egg_mode::{
    tweet::*
};
use std::{time::Duration, env, sync::Arc};
use tokio::time::delay_for;
use crate::structures::Config;
use dashmap::DashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static TWEET_MAP: Lazy<Arc<DashMap<u64, u64>>> = Lazy::new(|| {
    let tweet_map = DashMap::new();
    Arc::new(tweet_map)
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::generate(&args[1]).await;
    
    tokio::spawn(async move {
        delay_for(Duration::from_secs(30)).await;
        clear_cache().await
    });

    perform_retweet(&config).await;

    Ok(())
}

async fn clear_cache() {
    loop {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards?").as_secs();
    
        for guard in TWEET_MAP.iter() {
            if guard.value() < &since_epoch {
                TWEET_MAP.remove(guard.key());
            }
        }
        delay_for(Duration::from_secs(43200)).await;
    }
}

async fn perform_retweet(config: &Config) {
    loop {
        let mentions = mentions_timeline(&config.token).with_page_size(2);
        let (_mentions, feed) = match mentions.start().await {
            Ok(resp) => resp,
            Err(e) => {
                println!("There was an error when fetching the timeline!: {}", e);
                break;
            }
        };

        for status in feed.iter() {
            let start = SystemTime::now();
            let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards?").as_secs();

            if let Some(guard) = TWEET_MAP.get(&status.id) {
                if guard.value() < &since_epoch {
                    TWEET_MAP.remove(&status.id);
                }
            } else {
                let _ = retweet(status.id, &config.token).await;
                TWEET_MAP.insert(status.id, since_epoch + 3600);
            }
        }
        delay_for(Duration::from_secs(config.rt_delay)).await;
    }
}

