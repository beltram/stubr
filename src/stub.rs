use std::convert::TryInto;
use std::fs::OpenOptions;
use std::path::PathBuf;

use serde::export::TryFrom;
use wiremock::Mock;

use super::model::stub::Stub;

pub struct StubrMock(pub Mock);

impl TryFrom<PathBuf> for StubrMock {
    type Error = anyhow::Error;

    fn try_from(file: PathBuf) -> anyhow::Result<Self> {
        let file = OpenOptions::new().read(true).open(file)?;
        let stub: Stub = serde_json::from_reader(file)?;
        Ok(Self {
            0: stub.try_into()?,
        })
    }
}
