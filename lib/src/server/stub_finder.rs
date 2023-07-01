use std::{
    env,
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::{StubrError, StubrResult};
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

    pub fn find_all_stubs(from: &Path) -> impl Iterator<Item = PathBuf> {
        if from.exists() {
            if from.is_dir() {
                Self::find_all_stubs_under_dir(from).into_iter()
            } else {
                vec![from.to_path_buf()].into_iter()
            }
        } else {
            vec![].into_iter()
        }
    }

    fn find_all_stubs_under_dir(from: &Path) -> Vec<PathBuf> {
        let mut stubs = vec![];
        if let Ok(mut from) = read_dir(from) {
            while let Some(Ok(entry)) = from.next() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(OsStr::to_str) == Some(Self::JSON_EXTENSION) {
                    stubs.push(path)
                } else if path.is_dir() {
                    Self::find_all_stubs_under_dir(&path).into_iter().for_each(|s| stubs.push(s))
                }
            }
        }
        stubs
    }

    pub fn find_app(name: &str) -> StubrResult<PathBuf> {
        let pkg = env::var("CARGO_PKG_NAME")?;
        let out_dir = Self::output_dir().ok_or(StubrError::OutputDirFound)?;
        let app = out_dir.join(Self::LOCAL_DIR).join(pkg).join(name);
        app.exists().then(|| app).ok_or(StubrError::AppNotFound(name.to_string()))
    }

    pub fn output_dir() -> Option<PathBuf> {
        let var = Self::LIB_PATH_ENV_VAR;
        let result = env::var(var);
        let unix = env::var("LD_LIBRARY_PATH");
        let macos = env::var("DYLD_FALLBACK_LIBRARY_PATH");
        let out_dir = env::var("OUT_DIR");
        let a = env::var("CARGO_TARGET_TMPDIR");
        let b = env::var("CARGO_MANIFEST_DIR");
        let c = env::var("CARGO_BUILD_TARGET");
        let d = env::var("CARGO_BUILD_TARGET_DIR");
        let e = env::var("CARGO_BUILD_DEP_INFO_BASEDIR");
        let f = env::var("CARGO_TARGET_DIR");
        let g = env::var("CARGO_PRIMARY_PACKAGE");
        let lib_path = result.ok()?;
        lib_path
            .split(':')
            .map(PathBuf::from)
            .find_map(|p| Self::find_target(&p).or_else(|| Self::find_target(p.parent()?)))
    }

    fn find_target(path: &Path) -> Option<PathBuf> {
        let is_named = |p: &Path, name: &str| {
            p.file_name()
                .and_then(OsStr::to_str)
                .map(|n| n.split(';').collect_vec())
                .and_then(|v| v.first().map(|it| it.to_string()))
                == Some(name.to_string())
        };
        let debug = is_named(path, "debug");
        let target = path.parent().map(|p| is_named(p, "target")).unwrap_or_default();
        let found = debug && target;
        found.then_some(path.parent()?.to_path_buf())
    }
}

#[cfg(test)]
mod stub_finder_test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn should_find_just_json_files_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let files = StubFinder::find_all_stubs(&from).collect::<Vec<PathBuf>>();
        assert!(files.len().gt(&2));
        let file_names = files.iter().map(|it| it.file_name().unwrap().to_str().unwrap()).collect_vec();
        assert!(file_names.contains(&"valid.json"));
        assert!(file_names.contains(&"also_valid.json"));
    }

    #[test]
    fn should_find_all_files_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        let files = StubFinder::find_all_stubs(&from).collect::<Vec<PathBuf>>();
        assert_eq!(files.len(), 1);
        let file_names = files.iter().map(|it| it.file_name().unwrap().to_str().unwrap()).collect_vec();
        assert!(file_names.contains(&"valid.json"));
    }

    #[test]
    fn should_not_find_any_file_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        assert_eq!(StubFinder::find_all_stubs(&from).count(), 0);
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        assert_eq!(StubFinder::find_all_stubs(&from).count(), 0);
    }

    #[test]
    fn should_return_empty_vec_when_read_dir_fails() {
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = StubFinder::find_all_stubs_under_dir(&from);
        assert!(files.is_empty());
    }

    #[test]
    fn should_find_all_stubs_recursively() {
        let from = PathBuf::from("tests/stubs/recur");
        let files = StubFinder::find_all_stubs(&from).collect::<Vec<PathBuf>>();
        assert_eq!(files.len(), 3);
        let file_names = files.iter().map(|it| it.file_name().unwrap().to_str().unwrap()).collect_vec();
        assert!(file_names.contains(&"get.json"));
        assert!(file_names.contains(&"post.json"));
        assert!(file_names.contains(&"delete.json"));
    }
}
