use std::borrow::BorrowMut;
use std::ffi::OsString;

use http_types::Response;

use body::BodyVerifier;
use header::HeaderVerifier;
use status::StatusVerifier;

use crate::model::response::{template::data::RequestData, ResponseStub};

use super::req::StdRequest;

mod body;
mod header;
mod status;

#[derive(Debug)]
pub struct StdResponse(pub Response);

impl Default for StdResponse {
    fn default() -> Self {
        Self(Response::new(200))
    }
}

trait Verifier<'a> {
    fn verify(self, stub: &'a ResponseStub, name: &'a str, req: &'a RequestData, resp: &'a mut StdResponse);
}

pub struct RequestAndStub {
    pub req: StdRequest,
    pub stub: ResponseStub,
    pub name: OsString,
}

impl RequestAndStub {
    pub fn verify(mut self, mut resp: StdResponse) {
        let name = self.name().to_string();
        let req_data = RequestData::from(self.req.0.borrow_mut());
        HeaderVerifier.verify(&self.stub, &name, &req_data, &mut resp);
        StatusVerifier.verify(&self.stub, &name, &req_data, &mut resp);
        BodyVerifier.verify(&self.stub, &name, &req_data, &mut resp);
    }

    fn name(&self) -> &str {
        self.name.to_str().unwrap_or_default()
    }
}

#[cfg(test)]
mod resp_verify_tests {
    use http_types::{Request, Response};

    use crate::model::response::ResponseStub;

    use super::*;

    #[test]
    fn should_verify() {
        let stub = ResponseStub {
            status: Some(200),
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let mut resp = StdResponse(Response::new(200));
        StatusVerifier.verify(&stub, "ok", &RequestData::from(&mut req), &mut resp);
    }
}
