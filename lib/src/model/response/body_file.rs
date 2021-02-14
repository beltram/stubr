use std::{convert::TryFrom, fs::File, io::{BufReader, Read}, path::PathBuf};

use serde_json::Value;
use wiremock::ResponseTemplate;

use super::ResponseAppender;

pub struct BodyFile {
    file: PathBuf
}

impl BodyFile {
    const JSON_EXT: &'static str = "json";
    const TEXT_EXT: &'static str = "txt";

    fn read_file(&self) -> anyhow::Result<BufReader<File>> {
        File::open(self.file.as_path())
            .map(BufReader::new)
            .map_err(anyhow::Error::msg)
    }

    fn has_file_extension(&self, extension: &str) -> bool {
        self.file.extension()
            .map(|it| it.eq(extension))
            .unwrap_or_default()
    }

    fn maybe_as_json(&self) -> anyhow::Result<Value> {
        if self.has_file_extension(Self::JSON_EXT) {
            self.read_file()
                .and_then(|it| serde_json::from_reader(it).map_err(anyhow::Error::msg))
        } else { anyhow::Result::Err(anyhow::Error::msg("file should have json extension")) }
    }

    fn maybe_as_text(&self) -> anyhow::Result<String> {
        if self.has_file_extension(Self::TEXT_EXT) {
            let mut buff = String::new();
            self.read_file()?.read_to_string(&mut buff)?;
            Ok(buff)
        } else { anyhow::Result::Err(anyhow::Error::msg("file should have json extension")) }
    }
}

impl ResponseAppender for BodyFile {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Ok(json) = self.maybe_as_json() {
            resp = resp.set_body_json(json);
        } else if let Ok(text) = self.maybe_as_text() {
            resp = resp.set_body_string(text);
        } else {
            resp = ResponseTemplate::new(500)
        }
        resp
    }
}

impl TryFrom<Option<&String>> for BodyFile {
    type Error = anyhow::Error;

    fn try_from(maybe_path: Option<&String>) -> anyhow::Result<Self> {
        maybe_path
            .map(PathBuf::from)
            .map(|file| Self { file })
            .ok_or_else(|| anyhow::Error::msg("Invalid body file path"))
    }
}