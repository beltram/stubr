use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{Mock, MockBuilder, ResponseTemplate};

use super::{request::RequestDto, response::ResponseDto};

#[derive(Deserialize, Debug)]
pub struct StubDto {
    pub uuid: Option<String>,
    request: RequestDto,
    pub response: ResponseDto,
}

impl TryFrom<StubDto> for Mock {
    type Error = anyhow::Error;

    fn try_from(stub: StubDto) -> Result<Self, Self::Error> {
        let response = ResponseTemplate::from(&stub);
        Ok(MockBuilder::try_from(stub.request)?.respond_with(response))
    }
}
