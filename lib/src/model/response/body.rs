use std::{
    convert::TryFrom,
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
    str::from_utf8,
};

use serde::Deserialize;
use serde_json::Value;
use wiremock::ResponseTemplate;

use crate::model::response::template::{data::HandlebarsData, HandlebarTemplatable};

use super::{body_file::BodyFile, ResponseAppender};

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyDto {
    /// plain text body
    pub body: Option<String>,
    /// json body
    pub json_body: Option<Value>,
    /// relative path to raw body content
    pub body_file_name: Option<String>,
}

impl HandlebarTemplatable for BodyDto {
    fn register_template(&self) {
        if let Some(body) = self.body.as_ref() {
            self.register(body, body);
        } else if let Some(json_body) = self.json_body.as_ref().map(ToString::to_string) {
            self.register(json_body.as_str(), json_body.clone());
        } else if let Some(path) = self.body_file_name.as_ref() {
            if let Some(content) = self.read_file() {
                self.register(path, content);
            }
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
        } else if let Some(path) = self.body_file_name.as_ref() {
            template = BodyFile::from((PathBuf::from(path), self.render(path, data))).add(template);
        }
        template
    }
}

impl BodyDto {
    fn read_file(&self) -> Option<String> {
        self.body_file_name.as_ref()
            .map(PathBuf::from)
            .and_then(|file| OpenOptions::new().read(true).open(file).ok())
            .and_then(|mut file| {
                let mut buf = vec![];
                file.read_to_end(&mut buf).map(|_| buf).ok()
            })
            .and_then(|bytes| from_utf8(bytes.as_slice()).map(|it| it.to_string()).ok())
    }
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