use serde::{Deserialize, Serialize};
use std::fs;
use std::io::BufReader;
use egg_mode::Token;
use fs::File;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub rt_delay: Option<u64>,
    pub access_key: Option<String>,
    pub access_token: Option<String>,
    pub username: Option<String>,
    pub user_id: Option<u64>
}

pub fn read_creds(path: &str) -> Result<Credentials, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let info: Credentials = serde_json::from_reader(reader).unwrap();

    Ok(info)
}

pub fn add_extra_creds(path: &str, token: &Token, user_id: &u64, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut creds = read_creds(path).unwrap();

    match token {
        egg_mode::Token::Access {
            access: ref access_token,
            ..
        } => {
            creds.access_key = Some(access_token.key.to_string());
            creds.access_token = Some(access_token.secret.to_string());
            creds.username = Some(username.to_string());
            creds.user_id = Some(user_id.clone());
        }
        _ => println!("No token found!"),
    }

    serde_json::to_writer_pretty(File::create(path).unwrap(), &creds).unwrap();

    Ok(())
}

pub fn remove_user_creds(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut creds = read_creds(path).unwrap();

    creds.access_key = None;
    creds.access_token = None;
    creds.username = None;
    creds.user_id = None;

    serde_json::to_writer_pretty(File::create(path).unwrap(), &creds).unwrap();

    Ok(())
}