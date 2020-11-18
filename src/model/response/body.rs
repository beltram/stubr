use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::Value;
use wiremock::ResponseTemplate;

use super::body_file::BodyFile;
use super::ResponseAppender;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BodyDto {
    /// plain text body
    pub body: Option<String>,
    /// json body
    pub json_body: Option<Value>,
    /// relative path to raw body content
    pub body_file_name: Option<String>,
}

impl ResponseAppender for BodyDto {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(text) = self.body.as_ref() {
            resp = resp.set_body_string(text);
        }
        if let Some(json) = self.json_body.as_ref() {
            resp = resp.set_body_json(json)
        }
        if let Ok(body_file) = BodyFile::try_from(self.body_file_name.as_ref()) {
            resp = body_file.add(resp)
        }
        resp
    }
}