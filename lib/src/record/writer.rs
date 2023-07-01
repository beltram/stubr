use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir_all, File},
    hash::{Hash, Hasher},
    path::PathBuf,
    str::FromStr,
};

use http_types::Url;
use log::info;

use crate::{model::JsonStub, server::stub_finder::StubFinder, StubrError, StubrResult};

pub(crate) struct StubWriter {
    pub(crate) stub: JsonStub,
}

impl StubWriter {
    const RECORDED_TEST_DIR: &'static str = "stubs";

    pub(crate) fn write(&self, host: &str, output: Option<&PathBuf>) -> StubrResult<PathBuf> {
        let output = self.try_output_and_create(host, output)?;
        let path = output.join(self.stub_name());
        let file = File::create(&path)?;
        serde_json::to_writer_pretty(&file, &self.stub)?;
        info!("Recorded stub in {:?}", &file);
        Ok(path)
    }

    pub(crate) fn stub_name(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.stub.hash(&mut hasher);
        format!("{}{}.json", self.base_path().unwrap_or_default(), hasher.finish())
    }

    fn base_path(&self) -> Option<String> {
        self.stub.http_request.as_ref().and_then(|r| {
            r.url
                .url_path
                .as_ref()
                .map(|it| it.strip_prefix('/').unwrap_or(it))
                .map(|it| it.replace('/', "-"))
                .map(|it| format!("{it}-"))
        })
    }

    fn try_output_and_create(&self, host: &str, output: Option<&PathBuf>) -> StubrResult<PathBuf> {
        let output = output
            .map(|it| it.to_path_buf())
            .or_else(|| Self::default_output())
            .ok_or(StubrError::OutputDirFound);
        let output = output?.join(self.dir_name(host));
        if !output.exists() {
            create_dir_all(&output)?;
        }
        Ok(output)
    }

    fn dir_name(&self, host: &str) -> String {
        if host == "127.0.0.1" || host == "localhost" {
            String::from("localhost")
        } else if let Ok(url) = Url::from_str(host) {
            let (host, port) = url
                .host_str()
                .filter(|&h| h != "127.0.0.1")
                .map(|h| h.replace(|c: char| !c.is_alphanumeric(), "."))
                .map(|h| {
                    if let Some(p) = url.port() {
                        (h, format!("-{p}"))
                    } else {
                        (h, String::new())
                    }
                })
                .unwrap_or(("localhost".to_string(), String::new()));
            format!("{host}{port}")
        } else {
            String::from("default")
        }
    }

    fn default_output() -> Option<PathBuf> {
        Some(StubFinder::output_dir()?.join(Self::RECORDED_TEST_DIR))
    }
}
