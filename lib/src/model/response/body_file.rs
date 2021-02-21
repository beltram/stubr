use std::{convert::TryFrom, fs::File, io::{BufReader, Read}, path::PathBuf};

use anyhow::Error;
use serde_json::Value;
use wiremock::ResponseTemplate;

use super::ResponseAppender;

pub struct BodyFile {
    file: PathBuf,
    maybe_content: Option<String>,
}

impl BodyFile {
    const JSON_EXT: &'static str = "json";
    const TEXT_EXT: &'static str = "txt";

    fn maybe_as_json(&self) -> Option<Value> {
        if self.has_file_extension(Self::JSON_EXT) {
            self.read_from_json_content()
                .or_else(|| self.read_json_from_file())
        } else { None }
    }

    fn read_from_json_content(&self) -> Option<Value> {
        self.maybe_content.as_ref()
            .and_then(|content| serde_json::from_str::<Value>(content.as_str()).ok())
    }

    fn read_json_from_file(&self) -> Option<Value> {
        self.read_file()
            .and_then(|it| serde_json::from_reader(it).ok())
    }

    fn maybe_as_text(&self) -> Option<String> {
        if self.has_file_extension(Self::TEXT_EXT) {
            self.maybe_content.to_owned()
                .or_else(|| self.read_from_text_file())
        } else { None }
    }

    fn read_from_text_file(&self) -> Option<String> {
        let mut buff = String::new();
        self.read_file()
            ?.read_to_string(&mut buff).ok()
            .map(|_| buff)
    }

    fn has_file_extension(&self, extension: &str) -> bool {
        self.file.extension()
            .map(|it| it.eq(extension))
            .unwrap_or_default()
    }

    fn read_file(&self) -> Option<BufReader<File>> {
        File::open(self.file.as_path())
            .map(BufReader::new)
            .ok()
    }
}

impl ResponseAppender for BodyFile {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(json) = self.maybe_as_json() {
            resp = resp.set_body_json(json);
        } else if let Some(text) = self.maybe_as_text() {
            resp = resp.set_body_string(text);
        } else {
            resp = ResponseTemplate::new(500)
        }
        resp
    }
}

impl TryFrom<Option<&String>> for BodyFile {
    type Error = Error;

    fn try_from(maybe_path: Option<&String>) -> anyhow::Result<Self> {
        maybe_path
            .map(PathBuf::from)
            .map(|file| Self { file, maybe_content: None })
            .ok_or_else(|| Error::msg("Invalid body file path"))
    }
}

impl From<(PathBuf, String)> for BodyFile {
    fn from((file, content): (PathBuf, String)) -> Self {
        Self { file, maybe_content: Some(content) }
    }
}