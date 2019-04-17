use failure::Error;
use serde::{Deserialize, Serialize};
use url::Url;

pub struct SolutionClient<'a> {
    server_url: &'a Url,
}

impl<'a> SolutionClient<'a> {
    pub(super) fn new(server_url: &Url) -> SolutionClient {
        SolutionClient { server_url }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    pub judge_id: u64,
    pub user_id: String,
    pub problem_id: String,
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

impl<'a> SolutionClient<'a> {
    /// Get all solution records.
    pub fn find_all(&self, request: &FindAllRequest) -> Result<Vec<Solution>, Error> {
        let mut url = self.server_url.join("solutions")?;
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
    use super::FindAllRequest;
    use super::Solution;
    use super::SolutionClient;
    use url::Url;

    #[test]
    fn test_find_all() {
        let mock = mockito::mock("GET", "/solutions?size=10")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body(include_str!("../resource/solutions.json"))
            .create();

        let url = Url::parse(&mockito::server_url()).unwrap();
        let client: SolutionClient = SolutionClient::new(&url);
        let solutions: Vec<Solution> = client
            .find_all(&FindAllRequest::default().set_size(10))
            .unwrap();

        assert_eq!(
            vec![Solution {
                judge_id: 3491131,
                user_id: "irtfrm".to_string(),
                problem_id: "ALDS1_1_D".to_string(),
            }],
            solutions
        );
        mock.assert();
    }
}
