use std::convert::{TryFrom, TryInto};

use serde::Deserialize;
use wiremock::{Mock, MockBuilder};

use super::request::Request;
use super::response::Response;

#[derive(Deserialize, Debug)]
pub struct Stub {
    request: Request,
    response: Response,
}

impl TryFrom<Stub> for Mock {
    type Error = anyhow::Error;

    fn try_from(stub: Stub) -> Result<Self, Self::Error> {
        let builder: MockBuilder = stub.request.try_into()?;
        Ok(builder.respond_with(stub.response.try_into()?))
    }
}
