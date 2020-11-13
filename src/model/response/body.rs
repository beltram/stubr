use serde::Deserialize;
use wiremock::ResponseTemplate;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BodyDto {
    pub body: Option<String>,
}

impl BodyDto {
    pub fn add_to_response(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(text) = self.body.as_ref() {
            resp = resp.set_body_string(text);
        }
        resp
    }
}