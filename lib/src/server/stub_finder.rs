use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf as StdPathBuf};

use async_std::{fs, path::PathBuf, stream::{self, Stream}};
use futures::StreamExt;
use itertools::Itertools;

pub struct StubFinder;

impl StubFinder {
    const LOCAL_DIR: &'static str = "stubr";
    const JSON_EXTENSION: &'static str = "json";

    #[cfg(target_os = "macos")]
    const LIB_PATH_ENV_VAR: &'static str = "DYLD_FALLBACK_LIBRARY_PATH";
    #[cfg(target_os = "windows")]
    const LIB_PATH_ENV_VAR: &'static str = "PATH";
    #[cfg(target_os = "linux")]
    const LIB_PATH_ENV_VAR: &'static str = "LD_LIBRARY_PATH";

    pub async fn find_all_stubs(from: &PathBuf) -> impl Stream<Item=PathBuf> {
        if from.exists().await {
            if from.is_dir().await {
                stream::from_iter(Self::find_all_stubs_under_dir(from).await)
            } else { stream::from_iter(vec![from.to_path_buf()]) }
        } else { stream::from_iter(vec![]) }
    }

    async fn find_all_stubs_under_dir(from: &PathBuf) -> Vec<PathBuf> {
        let mut stubs = vec![];
        if let Ok(mut from) = fs::read_dir(from).await {
            while let Some(Ok(entry)) = from.next().await {
                let path = entry.path();
                if path.is_file().await && path.extension().and_then(|ext| ext.to_str()) == Some(Self::JSON_EXTENSION) {
                    stubs.push(path)
                }
            }
        }
        stubs
    }

    pub fn find_app(name: &str) -> StdPathBuf {
        Self::output_dir()
            .join(Self::LOCAL_DIR)
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join(name)
    }

    pub fn output_dir() -> StdPathBuf {
        env::var(Self::LIB_PATH_ENV_VAR).ok()
            .and_then(|v| v.split(':').map(StdPathBuf::from).find(|p| Self::is_target_debug(p)))
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

#[cfg(test)]
mod stub_finder_test {
    use itertools::Itertools;

    use super::*;

    #[async_std::test]
    async fn should_find_just_json_files_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let files = StubFinder::find_all_stubs(&from).await.collect::<Vec<PathBuf>>().await;
        assert_eq!(files.len(), 2);
        let file_names = files.iter()
            .map(|it| it.file_name().unwrap().to_str().unwrap())
            .collect_vec();
        assert!(file_names.contains(&"valid.json"));
        assert!(file_names.contains(&"also_valid.json"));
    }

    #[async_std::test]
    async fn should_find_all_files_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        let files = StubFinder::find_all_stubs(&from).await.collect::<Vec<PathBuf>>().await;
        assert_eq!(files.len(), 1);
        let file_names = files.iter()
            .map(|it| it.file_name().unwrap().to_str().unwrap())
            .collect_vec();
        assert!(file_names.contains(&"valid.json"));
    }

    #[async_std::test]
    async fn should_not_find_any_file_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let files = StubFinder::find_all_stubs(&from).await.collect::<Vec<PathBuf>>().await;
        assert!(files.is_empty());
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = StubFinder::find_all_stubs(&from).await.collect::<Vec<PathBuf>>().await;
        assert!(files.is_empty());
    }

    #[async_std::test]
    async fn should_return_empty_vec_when_read_dir_fails() {
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = StubFinder::find_all_stubs_under_dir(&from).await;
        assert!(files.is_empty());
    }
}