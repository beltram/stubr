use std::{env::current_dir, ffi::{OsStr, OsString}, fs::OpenOptions, path::PathBuf};

use super::{super::model::JsonStub, VerifyExcept};

pub(crate) struct ProducerStubFinder;

impl ProducerStubFinder {
    pub(crate) fn find_stubs<N>(except: impl VerifyExcept<N>) -> Vec<(JsonStub, OsString)> {
        Self::stub_dir()
            .and_then(|dir| dir.read_dir().ok())
            .map(|d| d.filter_map(Result::ok).map(|dir| dir.path()))
            .map(Self::map_json_stub)
            .map(|stubs| stubs
                .filter(|(_, n)| n.to_str()
                    .map(|s| s.trim_end_matches(".json"))
                    .map(str::to_string)
                    .map(|s| !except.call(s))
                    .unwrap_or_default())
                .collect()
            )
            .unwrap_or_default()
    }

    fn stub_dir() -> Option<PathBuf> {
        current_dir().map(|it| it.join("stubs")).ok()
    }

    fn map_json_stub(files: impl Iterator<Item=PathBuf>) -> impl Iterator<Item=(JsonStub, OsString)> {
        files
            .filter_map(|path| path.file_name().map(OsStr::to_owned).zip(OpenOptions::new().read(true).open(path).ok()))
            .filter_map(|(name, file)| serde_json::from_reader(file).ok().zip(Some(name)))
    }
}