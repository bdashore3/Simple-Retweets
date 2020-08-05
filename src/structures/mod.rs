use egg_mode::{KeyPair, Token, auth};
use crate::helpers::credentials_helper;
use auth::access_token;

// A smaller form of the Credentials struct containing the necessary info for this run
pub struct Config {
    pub token: Token,
    pub user_id: u64,
    pub screen_name: String,
    pub rt_delay: u64,
    pub page_size: i32
}

impl Config {
    /*
     * Taken from egg-mode's examples
     * Generates a config struct. If there's an error in authentication, retry
     */
    pub async fn generate(path: &str) -> Self {
        let first_attempt = Config::generate_inner(path).await;
        if let Some(conf) = first_attempt {
            return conf
        }

        Config::generate_inner(path).await.unwrap()
    }

    async fn generate_inner(path: &str) -> Option<Self> {
        let creds = match credentials_helper::read_creds(path) {
            Ok(creds) => creds,
            Err(_e) => {
                panic!("You need an info.json file! \nPlease create one using the github sample and run the program with the path as an argument!");
            }
        };

        let con_token = KeyPair::new(creds.consumer_key, creds.consumer_secret);

        let token: Token;
        let user_id: u64;
        let username: String;
        let rt_delay = creds.rt_delay.unwrap_or(180);
        let page_size = creds.page_size.unwrap_or(5);

        /*
         * If there is an access key, test it against twitter's servers
         *
         * When there's an error, return None and re-run the function 
         * to reauthenticate the user
         */
        if creds.access_key.is_some() {
            user_id = creds.user_id.unwrap();
            username = creds.username.unwrap();

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
                    screen_name: username,
                    rt_delay: rt_delay,
                    page_size: page_size
                })
            }
        } else {
            let request_token = auth::request_token(&con_token, "oob").await.unwrap();

            println!("Go to the following URL, sign in, and give me the PIN that comes back:");
            println!("{}", auth::authorize_url(&request_token));

            let mut pin = String::new();
            std::io::stdin().read_line(&mut pin).unwrap();

            let tok_result = match access_token(con_token, &request_token, pin).await {
                Ok(token) => token,
                Err(_e) => {
                    panic!("There was an error in authentication! Please make sure you entered the right pin from twitter!");
                }
            };

            credentials_helper::add_extra_creds(&path, &tok_result.0, &tok_result.1, &tok_result.2).unwrap();

            token = tok_result.0;
            user_id = tok_result.1;
            username = tok_result.2;

            println!("Welcome {}! Let's get started!", username);

            Some(Config {
                token: token,
                user_id: user_id,
                screen_name: username,
                rt_delay: rt_delay,
                page_size: page_size
            })
        }
    }
}