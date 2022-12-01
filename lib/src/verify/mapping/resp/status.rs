use crate::model::response::{template::data::RequestData, ResponseStub};

use super::{StdResponse, Verifier};

pub struct StatusVerifier;

impl Verifier<'_> for StatusVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, _req: &'_ RequestData, resp: &'_ mut StdResponse) {
        let expected = stub.status();
        let actual = u16::from(resp.0.status());
        assert_eq!(
            actual, expected,
            "Verification failed for stub '{}'. Expected response status to be '{}' but was '{}'",
            name, expected, actual
        );
    }
}

#[cfg(test)]
mod status_verify_tests {
    use http_types::{Request, Response};

    use super::*;

    #[test]
    fn should_verify_200() {
        let stub = ResponseStub {
            status: Some(200),
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let req = RequestData::from(&mut req);
        let mut resp = StdResponse(Response::new(200));
        StatusVerifier.verify(&stub, "200", &req, &mut resp);
    }

    #[should_panic(expected = "Verification failed for stub '200'. Expected response status to be '200' but was '201'")]
    #[test]
    fn verify_should_fail_when_wrong_status_returned() {
        let stub = ResponseStub {
            status: Some(200),
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let req = RequestData::from(&mut req);
        let mut resp = StdResponse(Response::new(201));
        StatusVerifier.verify(&stub, "200", &req, &mut resp);
    }
}
