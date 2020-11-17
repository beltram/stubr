use serde::Deserialize;
use serde_json::Value;
use wiremock::ResponseTemplate;

use super::ResponseAppender;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BodyDto {
    /// plain text body
    pub body: Option<String>,
    /// json body
    pub json_body: Option<Value>,
}

impl ResponseAppender for BodyDto {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(text) = self.body.as_ref() {
            resp = resp.set_body_string(text);
        }
        if let Some(json) = self.json_body.as_ref() {
            resp = resp.set_body_json(json)
        }
        resp
    }
}