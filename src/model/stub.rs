use std::convert::{TryFrom, TryInto};

use serde::Deserialize;
use wiremock::{Mock, MockBuilder};

use super::request::Request;
use super::response::ResponseDto;

#[derive(Deserialize, Debug)]
pub struct Stub {
    request: Request,
    response: ResponseDto,
}

impl TryFrom<Stub> for Mock {
    type Error = anyhow::Error;

    fn try_from(stub: Stub) -> Result<Self, Self::Error> {
        Ok(MockBuilder::try_from(stub.request)?.respond_with(stub.response.try_into()?))
    }
}
