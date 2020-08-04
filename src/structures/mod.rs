use egg_mode::{KeyPair, Token, auth};
use crate::helpers::credentials_helper;
use auth::access_token;

pub struct Config {
    pub token: Token,
    pub user_id: u64,
    pub screen_name: String
}

impl Config {
    pub async fn generate(path: &str) -> Self {
        let first_attempt = Config::generate_inner(path).await;
        if let Some(conf) = first_attempt {
            return conf
        }

        Config::generate_inner(path).await.unwrap()
    }

    async fn generate_inner(path: &str) -> Option<Self> {
        let creds = credentials_helper::read_creds(path).unwrap();

        let con_token = KeyPair::new(creds.consumer_key, creds.consumer_secret);

        let user_id: u64;
        let username: String;
        let token: Token;

        if creds.access_key.is_some() {
            username = creds.username.unwrap();
            user_id = creds.user_id.unwrap();

            let access_token = KeyPair::new(
                creds.access_token.unwrap(),
                creds.access_key.unwrap()
            );
            token = Token::Access {
                consumer: con_token,
                access: access_token
            };

            if let Err(e) = auth::verify_tokens(&token).await {
                println!("There was an error when authenticating!: {:?}", e);
                println!("We need to reauthenticate");

                credentials_helper::remove_user_creds(path).unwrap();

                None
            } else {
                println!("Welcome back {}! \n", username);

                Some(Config {
                    token: token,
                    user_id: user_id,
                    screen_name: username
                })
            }
        } else {
            let request_token = auth::request_token(&con_token, "oob").await.unwrap();

            println!("Go to the following URL, sign in, and give me the PIN that comes back:");
            println!("{}", auth::authorize_url(&request_token));

            let mut pin = String::new();
            std::io::stdin().read_line(&mut pin).unwrap();

            let tok_result =
                access_token(con_token, &request_token, pin).await.unwrap();

            credentials_helper::add_extra_creds(&path, &tok_result.0, &tok_result.1, &tok_result.2).unwrap();

            token = tok_result.0;
            user_id = tok_result.1;
            username = tok_result.2;

            println!("Welcome {}! Let's get started!", username);

            Some(Config {
                token: token,
                user_id: user_id,
                screen_name: username
            })
        }
    }
}