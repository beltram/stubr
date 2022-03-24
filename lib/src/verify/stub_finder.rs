use std::{
    env::current_dir,
    ffi::OsString,
    fs::OpenOptions,
    path::PathBuf,
};

use super::super::model::JsonStub;

pub(crate) struct ProducerStubFinder;

impl ProducerStubFinder {
    pub(crate) fn find_stubs() -> Vec<(JsonStub, OsString)> {
        Self::stub_dir()
            .and_then(|d| d.read_dir().ok())
            .map(|d| d.filter_map(Result::ok).map(|dir| dir.path()).collect())
            .map(Self::map_json_stub)
            .unwrap_or_default()
    }

    fn stub_dir() -> Option<PathBuf> {
        current_dir().map(|it| it.join("stubs")).ok()
    }

    fn map_json_stub(files: Vec<PathBuf>) -> Vec<(JsonStub, OsString)> {
        files.iter()
            .filter_map(|path| OpenOptions::new().read(true).open(path).ok().zip(path.file_name()))
            .filter_map(|(file, name)| serde_json::from_reader(file).ok().zip(Some(name.to_os_string())))
            .collect()
    }
}