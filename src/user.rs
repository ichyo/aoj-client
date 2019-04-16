use failure::Error;
use serde::Deserialize;
use url::Url;

pub struct UserClient<'a> {
    server_url: &'a Url,
}

impl<'a> UserClient<'a> {
    pub(super) fn new(server_url: &Url) -> UserClient {
        UserClient { server_url }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    name: String,
    affiliation: String,
}

#[derive(Default)]
pub struct FindAllRequest {
    page: Option<usize>,
    size: Option<usize>,
}

impl FindAllRequest {
    pub fn new() -> FindAllRequest {
        FindAllRequest::default()
    }

    pub fn set_page(&mut self, page: usize) -> &mut FindAllRequest {
        self.page = Some(page);
        self
    }

    pub fn set_size(&mut self, size: usize) -> &mut FindAllRequest {
        self.size = Some(size);
        self
    }
}

impl<'a> UserClient<'a> {
    /// Get all users among several pages.
    pub fn find_all(&self, request: &FindAllRequest) -> Result<Vec<User>, Error> {
        let mut url = self.server_url.join("users")?;
        if let Some(page) = request.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }
        if let Some(size) = request.size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        let users = reqwest::get(url.as_str())?.json()?;
        Ok(users)
    }

    /// Get a user information by specifying user ID.
    pub fn find_by_id(&self, user_id: &str) -> Result<User, Error> {
        let url = self.server_url.join("users/")?.join(user_id)?;
        let user: User = reqwest::get(url.as_str())?.json()?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::FindAllRequest;
    use super::User;
    use super::UserClient;
    use url::Url;

    #[test]
    fn test_find_all() {
        let mock = mockito::mock("GET", "/users?size=10")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body(include_str!("../resource/users.json"))
            .create();

        let url = Url::parse(&mockito::server_url()).unwrap();
        let client: UserClient = UserClient::new(&url);
        let users: Vec<User> = client
            .find_all(&FindAllRequest::default().set_size(10))
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

    #[test]
    fn test_find_by_id() {
        let mock = mockito::mock("GET", "/users/ichyo")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body(include_str!("../resource/users/ichyo.json"))
            .create();

        let url = Url::parse(&mockito::server_url()).unwrap();
        let client: UserClient = UserClient::new(&url);
        let user: User = client.find_by_id("ichyo").unwrap();

        assert_eq!(
            User {
                id: "ichyo".to_string(),
                name: "ichyo".to_string(),
                affiliation: "AOJ-ICPC".to_string(),
            },
            user
        );
        mock.assert();
    }
}
