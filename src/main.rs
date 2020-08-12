mod helpers;
mod structures;

use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    env,
    sync::Arc
};
use egg_mode::tweet::{retweet, mentions_timeline, like};
use tokio::time::delay_for;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use crate::structures::Config;
use rust_clock::Clock;

static TWEET_MAP: Lazy<Arc<DashMap<u64, u64>>> = Lazy::new(|| {
    let tweet_map = DashMap::new();
    Arc::new(tweet_map)
});

pub type BotResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> BotResult<()> {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let first_err_string = "You need an info.json file!";
        let second_err_string = "Please create one using the github sample and run the program with the path as an argument!";
        eprintln!("{} \n{}", first_err_string, second_err_string);
        return Ok(());
    }

    let config = Config::generate(&args[1]).await;

    // Informs the user of various parameters
    let mut rt_delay_clock = Clock::new();
    rt_delay_clock.set_time_secs(config.rt_delay as i64);

    let mut informative_string = String::new();
    informative_string.push_str(&format!("Your retweet delay is currently: {} \n", rt_delay_clock.get_time()));
    informative_string.push_str(&format!("With a page size of: {} \n\n", config.page_size));
    informative_string.push_str(&format!("If you want to change these values, please edit them in info.json \n"));

    println!("{} \n", informative_string);
    
    tokio::spawn(async move {
        delay_for(Duration::from_secs(30)).await;
        clear_cache().await
    });

    perform_retweet(&config).await;

    Ok(())
}

/*
 * Fetch the mentions timeline and do two things
 *
 * 1. If the tweet is new, store the id along with a timestamp in the cache
 * 2. Retweet the tweet
 *
 * Doing this prevents rate limiting when a user wants to poll the API every few seconds
 */
async fn perform_retweet(config: &Config) {
    loop {
        let mentions = mentions_timeline(&config.token).with_page_size(config.page_size);
        let (_mentions, feed) = match mentions.start().await {
            Ok(resp) => resp,
            Err(e) => {
                println!("There was an error when fetching the timeline!: {} \nPlease open a github issue with this output!", e);
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
                if status.in_reply_to_status_id.is_none() {
                    let _ = retweet(status.id, &config.token).await;
                    let _ = like(status.id, &config.token).await;

                    TWEET_MAP.insert(status.id, since_epoch + 3600);
                }
            }
        }
        delay_for(Duration::from_secs(config.rt_delay)).await;
    }
}

/*
 * If there are any remaining cached tweets, flush them out.
 * This function is rarely used on smaller retweet accounts with longer delays
 */
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