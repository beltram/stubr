use serde::Deserialize;
use wiremock::ResponseTemplate;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: u16,
}

impl From<Response> for ResponseTemplate {
    fn from(response: Response) -> Self {
        ResponseTemplate::new(response.status)
    }
}
