use std::{fs::OpenOptions, io::Read, path::PathBuf, str::from_utf8};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use wiremock::ResponseTemplate;

use super::{body_file::BodyFile, ResponseAppender};
use super::template::{data::HandlebarsData, HandlebarTemplatable};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BodyStub {
    /// plain text body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// json body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<Value>,
    /// relative path to raw body content
    #[serde(default, skip_serializing, deserialize_with = "deserialize_body_file")]
    pub body_file_name: Option<BodyFile>,
}

fn deserialize_body_file<'de, D>(path: D) -> Result<Option<BodyFile>, D::Error> where D: Deserializer<'de> {
    let body_file = String::deserialize(path).ok()
        .map(PathBuf::from)
        .map(|path| {
            let path_exists = path.exists();
            let extension = path.extension().and_then(|it| it.to_str()).map(|it| it.to_string());
            let content = OpenOptions::new().read(true).open(&path).ok()
                .and_then(|mut file| {
                    let mut buf = vec![];
                    file.read_to_end(&mut buf).map(|_| buf).ok()
                })
                .and_then(|bytes| from_utf8(bytes.as_slice()).map(|it| it.to_string()).ok())
                .unwrap_or_default();
            let path = path.to_str().map(|it| it.to_string()).unwrap_or_default();
            BodyFile { path_exists, path, extension, content }
        });
    Ok(body_file)
}

impl HandlebarTemplatable for BodyStub {
    fn register_template(&self) {
        if let Some(body) = self.body.as_ref() {
            self.register(body, body);
        } else if let Some(json_body) = self.json_body.as_ref().map(ToString::to_string) {
            self.register(json_body.as_str(), json_body.clone());
        } else if let Some(body_file) = self.body_file_name.as_ref() {
            self.register(body_file.path.as_str(), body_file.content.clone());
        }
    }

    fn render_response_template(&self, mut template: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate {
        if let Some(body) = self.body.as_ref() {
            template = template.set_body_string(self.render(body.as_str(), data));
        } else if let Some(json_body) = self.json_body.as_ref().map(ToString::to_string) {
            let rendered = self.render(json_body.as_str(), data);
            if let Ok(value) = serde_json::from_str::<Value>(rendered.as_str()) {
                template = template.set_body_json(value);
            }
        } else if let Some(body_file) = self.body_file_name.as_ref() {
            let rendered = self.render(body_file.path.as_str(), data);
            template = body_file.render_templated(template, rendered);
        }
        template
    }
}

impl ResponseAppender for BodyStub {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(text) = self.body.as_ref() {
            resp = resp.set_body_string(text);
        }
        if let Some(json) = self.json_body.as_ref() {
            resp = resp.set_body_json(json)
        }
        if let Some(body_file) = self.body_file_name.as_ref() {
            resp = body_file.add(resp)
        }
        resp
    }
}