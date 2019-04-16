mod user;

use failure::Error;
use url::Url;
use user::UserClient;

static DEFAULT_SERVER_URL: &str = "https://judgeapi.u-aizu.ac.jp";

pub struct Client {
    server_url: Url,
}

impl Default for Client {
    fn default() -> Client {
        Client {
            server_url: Url::parse(DEFAULT_SERVER_URL).expect("default server url should be valid"),
        }
    }
}

impl Client {
    pub fn new(server_url: &str) -> Result<Client, Error> {
        Ok(Client {
            server_url: Url::parse(server_url)?,
        })
    }

    pub fn user_client(&self) -> UserClient {
        UserClient::new(&self.server_url)
    }
}
