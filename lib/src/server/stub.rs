use std::{convert::TryFrom, fs::OpenOptions, path::PathBuf};

use wiremock::Mock;

use crate::Config;

use super::super::model::JsonStub;

pub struct StubrMock(pub Mock);

impl TryFrom<(&PathBuf, &Config)> for StubrMock {
    type Error = anyhow::Error;

    fn try_from((maybe_stub, config): (&PathBuf, &Config)) -> anyhow::Result<Self> {
        let file = OpenOptions::new().read(true).open(&maybe_stub)?;
        let stub: JsonStub = serde_json::from_reader(file)?;
        Ok(Self { 0: stub.try_creating_from(config)? })
    }
}
