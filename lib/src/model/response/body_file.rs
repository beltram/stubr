use crate::wiremock::ResponseTemplate;
use serde::Serialize;
use serde_json::Value;

use super::ResponseAppender;

#[derive(Serialize, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BodyFile {
    pub path_exists: bool,
    pub path: String,
    pub extension: Option<String>,
    pub content: String,
}

impl BodyFile {
    const JSON_EXT: &'static str = "json";
    const TEXT_EXT: &'static str = "txt";

    const BODY_FILE_NAME_PREFIX: &'static str = "STUBR_BODY_FILE_NAME_TEMPLATE_PREFIX_";

    fn maybe_as_json(&self) -> Option<Value> {
        self.extension
            .as_deref()
            .filter(|&ext| ext == Self::JSON_EXT)
            .and_then(|_| serde_json::from_str::<Value>(self.content.as_str()).ok())
    }

    fn maybe_as_text(&self) -> Option<String> {
        self.extension
            .as_deref()
            .filter(|&ext| ext == Self::TEXT_EXT)
            .map(|_| self.content.to_owned())
    }

    fn is_json(&self) -> bool {
        self.extension.as_deref().map(|ext| ext == Self::JSON_EXT).unwrap_or_default()
    }

    fn is_text(&self) -> bool {
        self.extension.as_deref().map(|ext| ext == Self::TEXT_EXT).unwrap_or_default()
    }

    pub(crate) fn canonicalize_path(&self) -> String {
        format!("{}{}", Self::BODY_FILE_NAME_PREFIX, self.path)
    }
}

impl BodyFile {
    pub fn render_templated(&self, resp: ResponseTemplate, content: String) -> ResponseTemplate {
        if let Some(content) = self.is_json().then_some(serde_json::from_str::<Value>(&content).ok()) {
            return resp.set_body_json(content);
        }
        if self.is_text() {
            return resp.set_body_string(content);
        }
        ResponseTemplate::new(500)
    }
}

impl ResponseAppender for BodyFile {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if !self.path_exists {
            resp = ResponseTemplate::new(500)
        } else if let Some(json) = self.maybe_as_json() {
            resp = resp.set_body_json(json);
        } else if let Some(text) = self.maybe_as_text() {
            resp = resp.set_body_string(text);
        } else {
            resp = ResponseTemplate::new(500)
        }
        resp
    }
}
