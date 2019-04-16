use failure::Error;
use serde::Deserialize;
use url::Url;

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
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    id: String,
    name: String,
    affiliation: String,
}

#[derive(Default)]
pub struct FindUsersRequest {
    page: Option<usize>,
    size: Option<usize>,
}

impl FindUsersRequest {
    pub fn new() -> FindUsersRequest {
        FindUsersRequest::default()
    }

    pub fn set_page(&mut self, page: usize) -> &mut FindUsersRequest {
        self.page = Some(page);
        self
    }

    pub fn set_size(&mut self, size: usize) -> &mut FindUsersRequest {
        self.size = Some(size);
        self
    }
}

impl Client {
    pub fn find_users(&self, request: &FindUsersRequest) -> Result<Vec<User>, Error> {
        let mut url = self.server_url.join("/users")?;
        if let Some(page) = request.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }
        if let Some(size) = request.size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        let users = reqwest::get(url.as_str())?.json()?;
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use super::FindUsersRequest;
    use super::User;

    #[test]
    fn test_find_users() {
        let mock = mockito::mock("GET", "/users?size=10")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body(include_str!("../resource/users.json"))
            .create();

        let client: Client = Client::new(&mockito::server_url()).unwrap();
        let users: Vec<User> = client
            .find_users(&FindUsersRequest::default().set_size(10))
            .unwrap();

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
