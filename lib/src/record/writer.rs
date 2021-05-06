use std::{
    collections::hash_map::DefaultHasher,
    env,
    ffi::OsStr,
    fs::{create_dir_all, File},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
};

use http_types::Url;
use itertools::Itertools;

use crate::model::JsonStub;

pub(crate) struct StubWriter {
    pub(crate) stub: JsonStub,
}

impl StubWriter {
    const RECORDED_TEST_DIR: &'static str = "stubs";

    #[cfg(target_os = "macos")]
    const LIB_PATH_ENV_VAR: &'static str = "DYLD_FALLBACK_LIBRARY_PATH";
    #[cfg(target_os = "windows")]
    const LIB_PATH_ENV_VAR: &'static str = "PATH";
    #[cfg(target_os = "linux")]
    const LIB_PATH_ENV_VAR: &'static str = "LD_LIBRARY_PATH";

    pub(crate) fn write(&self, host: &str, output: Option<PathBuf>) -> anyhow::Result<PathBuf> {
        let output = self.output_and_create(host, output);
        let file = output.join(self.stub_name());
        File::create(&file)
            .map_err(anyhow::Error::msg)
            .and_then(|f| serde_json::to_writer_pretty(&f, &self.stub).map_err(anyhow::Error::msg).map(|_| file))
            .map_err(anyhow::Error::msg)
    }

    fn stub_name(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.stub.hash(&mut hasher);
        format!("{}{}.json", self.base_path().unwrap_or_default(), hasher.finish())
    }

    fn base_path(&self) -> Option<String> {
        self.stub.request.url.url_path.as_ref()
            .map(|it| it.strip_prefix('/').unwrap_or(it))
            .map(|it| it.replace('/', "-"))
            .map(|it| format!("{}-", it))
    }

    fn output_and_create(&self, host: &str, output: Option<PathBuf>) -> PathBuf {
        let output = output.unwrap_or_else(Self::default_output).join(self.dir_name(host));
        if !output.exists() {
            create_dir_all(&output)
                .unwrap_or_else(|_| panic!("Failed creating recorded stubs output directory at '{:?}'", &output));
        }
        output
    }

    fn dir_name(&self, host: &str) -> String {
        if let Ok(url) = Url::from_str(&host) {
            let (host, port) = url.host_str()
                .filter(|&h| h != "127.0.0.1")
                .map(|h| h.replace(|c: char| !c.is_alphanumeric(), "."))
                .map(|h| {
                    url.port()
                        .map(|p| (h.clone(), format!("-{}", p.to_string())))
                        .unwrap_or_else(|| (h, String::new()))
                })
                .unwrap_or((String::from("localhost"), String::new()));
            format!("{}{}", host, port)
        } else {
            String::from("default")
        }
    }

    fn default_output() -> PathBuf {
        Self::target_dir().join(Self::RECORDED_TEST_DIR)
    }

    fn target_dir() -> PathBuf {
        env::var(Self::LIB_PATH_ENV_VAR).ok()
            .and_then(|v| v.split(':').map(PathBuf::from).find(|p| Self::is_target_debug(p)))
            .and_then(|p| p.parent().map(|it| it.to_path_buf()))
            .expect("Failed locating '/target' directory")
    }

    fn is_target_debug(path: &Path) -> bool {
        let is_named = |p: &Path, name: &str| {
            p.file_name()
                .and_then(OsStr::to_str)
                .map(|n| n.split(';').collect_vec())
                .and_then(|v| v.get(0).map(|it| it.to_string()))
                == Some(name.to_string())
        };
        let debug = is_named(path, "debug");
        let target = path.parent().map(|p| is_named(&p.to_path_buf(), "target")).unwrap_or_default();
        debug && target
    }
}