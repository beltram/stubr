use std::convert::TryInto;
use std::fs::OpenOptions;
use std::path::PathBuf;

use serde::export::TryFrom;
use wiremock::Mock;

use super::model::stub::StubDto;

pub struct StubrMock(pub Mock, pub PathBuf);

impl TryFrom<PathBuf> for StubrMock {
    type Error = anyhow::Error;

    fn try_from(maybe_stub: PathBuf) -> anyhow::Result<Self> {
        let file = OpenOptions::new().read(true).open(&maybe_stub)?;
        let stub: StubDto = serde_json::from_reader(file)?;
        Ok(Self {
            0: stub.try_into()?,
            1: maybe_stub,
        })
    }
}
