use serde::Deserialize;
use std::error::Error;

static DEFAULT_SERVER_URL: &str = "https://judgeapi.u-aizu.ac.jp";

pub struct Client {
    server_url: String,
}

impl Default for Client {
    fn default() -> Client {
        Client {
            server_url: DEFAULT_SERVER_URL.to_string(),
        }
    }
}

impl Client {
    pub fn new(server_url: String) -> Client {
        Client { server_url }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    id: String,
    name: String,
    affiliation: String,
}

impl Client {
    pub fn find_users(&self) -> Result<Vec<User>, Box<Error>> {
        let url = format!("{}/users", self.server_url);
        let users = reqwest::get(&url)?.json()?;
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use super::User;

    #[test]
    fn test_find_users() {
        let client = Client::new(mockito::server_url());
        let mock = mockito::mock("GET", "/users")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body(include_str!("../resource/users.json"))
            .create();

        let users = client.find_users().unwrap();
        assert_eq!(
            vec![User {
                id: "ichyo".to_string(),
                name: "ichyo".to_string(),
                affiliation: "AOJ-ICPC".to_string(),
            }],
            users
        );
        mock.assert();
    }
}
