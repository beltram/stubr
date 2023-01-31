use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir_all, File},
    hash::{Hash, Hasher},
    path::PathBuf,
    str::FromStr,
};

use http_types::Url;
use log::info;

use crate::{model::JsonStub, server::stub_finder::StubFinder};

pub(crate) struct StubWriter {
    pub(crate) stub: JsonStub,
}

impl StubWriter {
    const RECORDED_TEST_DIR: &'static str = "stubs";

    pub(crate) fn write(&self, host: &str, output: Option<&PathBuf>) -> anyhow::Result<PathBuf> {
        let output = self.output_and_create(host, output);
        let file = output.join(self.stub_name());
        File::create(&file)
            .map_err(anyhow::Error::msg)
            .and_then(|f| {
                serde_json::to_writer_pretty(&f, &self.stub)
                    .map_err(anyhow::Error::msg)
                    .map(|_| file)
            })
            .map(|p| {
                info!("Recorded stub in {:?}", p);
                p
            })
            .map_err(anyhow::Error::msg)
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

    fn output_and_create(&self, host: &str, output: Option<&PathBuf>) -> PathBuf {
        let output = output
            .map(|it| it.to_path_buf())
            .unwrap_or_else(Self::default_output)
            .join(self.dir_name(host));
        if !output.exists() {
            create_dir_all(&output).unwrap_or_else(|_| panic!("Failed creating recorded stubs output directory at '{:?}'", &output));
        }
        output
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

    fn default_output() -> PathBuf {
        StubFinder::output_dir().join(Self::RECORDED_TEST_DIR)
    }
}
