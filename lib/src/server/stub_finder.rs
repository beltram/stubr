use async_std::{fs, path::PathBuf};
use futures::StreamExt;

pub struct StubFinder;

impl StubFinder {
    const JSON_EXTENSION: &'static str = "json";

    pub async fn find_all_stubs(from: &PathBuf) -> Vec<PathBuf> {
        if from.exists().await {
            if from.is_dir().await {
                Self::find_all_stubs_under_dir(from).await
            } else { vec![from.to_path_buf()] }
        } else { vec![] }
    }

    async fn find_all_stubs_under_dir(from: &PathBuf) -> Vec<PathBuf> {
        let mut stubs = vec![];
        if let Ok(mut from) = fs::read_dir(from).await {
            while let Some(Ok(entry)) = from.next().await {
                let path = entry.path();
                let extension = path.extension().and_then(|it| it.to_str());
                if path.is_file().await && extension == Some(Self::JSON_EXTENSION) {
                    stubs.push(path)
                }
            }
        }
        stubs
    }
}


#[cfg(test)]
mod stub_finder_test {
    use itertools::Itertools;

    use super::*;

    #[async_std::test]
    async fn should_find_just_json_files_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let files = StubFinder::find_all_stubs(&from).await;
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
        let files = StubFinder::find_all_stubs(&from).await;
        assert_eq!(files.len(), 1);
        let file_names = files.iter()
            .map(|it| it.file_name().unwrap().to_str().unwrap())
            .collect_vec();
        assert!(file_names.contains(&"valid.json"));
    }

    #[async_std::test]
    async fn should_not_find_any_file_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let files = StubFinder::find_all_stubs(&from).await;
        assert!(files.is_empty());
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = StubFinder::find_all_stubs(&from).await;
        assert!(files.is_empty());
    }

    #[async_std::test]
    async fn should_return_empty_vec_when_read_dir_fails() {
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = StubFinder::find_all_stubs_under_dir(&from).await;
        assert!(files.is_empty());
    }
}