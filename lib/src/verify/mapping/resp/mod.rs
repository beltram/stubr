use std::ffi::OsString;

use http_types::Response;

use body::BodyVerifier;
use header::HeaderVerifier;
use status::StatusVerifier;

use crate::model::response::ResponseStub;

use super::req::StdRequest;

mod status;
mod header;
mod body;

pub struct StdResponse(pub Response);

trait Verifier<'a> {
    fn verify(stub: &'a ResponseStub, name: &'a str, req: &'a StdRequest, resp: &'a mut StdResponse);
}

pub struct RequestAndStub {
    pub req: StdRequest,
    pub stub: ResponseStub,
    pub name: OsString,
}

impl RequestAndStub {
    pub fn verify(self, mut resp: StdResponse) {
        StatusVerifier::verify(&self.stub, self.name(), &self.req, &mut resp);
        HeaderVerifier::verify(&self.stub, self.name(), &self.req, &mut resp);
        BodyVerifier::verify(&self.stub, self.name(), &self.req, &mut resp);
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
        let stub = ResponseStub { status: Some(200), ..Default::default() };
        let req = StdRequest(Request::get("http://localhost/"));
        let mut resp = StdResponse(Response::new(200));
        StatusVerifier::verify(&stub, "ok", &req, &mut resp);
    }
}