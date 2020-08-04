mod helpers;
mod structures;

use egg_mode::{
    tweet
};
use std::{time::Duration, env};
use tokio::time::delay_for;
use crate::structures::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    let config = Config::generate(&args[1]).await;

    tokio::spawn(async move {
        loop {
            println!("Loading the user's mentions timeline:");
            let mentions = tweet::mentions_timeline(&config.token).with_page_size(10);
            let (_mentions, feed) = match mentions.start().await {
                Ok(resp) => resp,
                Err(e) => {
                    println!("There was an error when fetching the timeline!: {}", e);
                    break;
                }
            };
            for status in feed.iter() {
                println!("{}", status.text);
            }
            delay_for(Duration::from_millis(10000)).await;
        }
    }).await?;

    Ok(())
}