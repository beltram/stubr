use std::path::PathBuf;

use itertools::Itertools;

pub struct StubFinder;

impl StubFinder {
    pub fn find_all_stubs(from: &PathBuf) -> Vec<PathBuf> {
        if from.exists() {
            if from.is_dir() {
                from.read_dir()
                    .map(|dir| dir.into_iter().flatten()
                        .map(|it| it.path())
                        .filter(|it| it.is_file())
                        .collect_vec())
                    .unwrap_or_default()
            } else { vec![from.to_path_buf()] }
        } else { vec![] }
    }
}


#[cfg(test)]
mod stub_finder_test {
    use itertools::Itertools;

    use super::*;

    #[async_std::test]
    async fn should_find_all_files_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let files = StubFinder::find_all_stubs(&from);
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
        let files = StubFinder::find_all_stubs(&from);
        assert_eq!(files.len(), 1);
        let file_names = files.iter()
            .map(|it| it.file_name().unwrap().to_str().unwrap())
            .collect_vec();
        assert!(file_names.contains(&"valid.json"));
    }

    #[async_std::test]
    async fn should_not_find_any_file_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let files = StubFinder::find_all_stubs(&from);
        assert!(files.is_empty());
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = StubFinder::find_all_stubs(&from);
        assert!(files.is_empty());
    }
}