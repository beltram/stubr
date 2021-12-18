use std::ffi::OsString;

use actix_http::{body::Body as ActixBody, Error as ActixError, Request};
use actix_service::{Service, ServiceFactory};
use actix_web::{
    App,
    dev::{ServiceRequest as ActixServiceRequest, ServiceResponse as ActixServiceResponse},
    test::{init_service, TestRequest},
};
use async_trait::async_trait;

use crate::model::JsonStub;

use super::{
    mapping::{req::StdRequest, resp::{RequestAndStub, StdResponse}},
    stub_finder::ProducerStubFinder,
    StubrVerify,
};

mod req;
mod resp;

#[async_trait(? Send)]
impl<T> StubrVerify for App<T, ActixBody> where T: ServiceFactory<
    Request=ActixServiceRequest,
    Response=ActixServiceResponse<ActixBody>,
    Error=ActixError,
    Config=(),
    InitError=(),
> {
    async fn verify(self) {
        let mut app = init_service(self).await;
        for (stub, name) in ProducerStubFinder::find_stubs() {
            ActixApp(&mut app).verify(stub, name).await;
        };
    }
}

pub struct ActixApp<'a, A>(pub &'a mut A);

impl<A> ActixApp<'_, A> where A: Service<Request=Request, Response=ActixServiceResponse, Error=ActixError> {
    pub async fn verify(self, stub: JsonStub, name: OsString) {
        let mut req = StdRequest::from(&stub);
        let test_req = TestRequest::from(&mut req).to_request();
        let resp: StdResponse = self.0.call(test_req).await
            .unwrap_or_else(|_| panic!("Failed verifying stub {:?}", name))
            .into();
        RequestAndStub { req, stub: stub.response, name }.verify(resp);
    }
}

#[cfg(test)]
mod actix_verify_tests {
    use actix_web::{App, HttpResponse, web};

    use crate::model::{request::{method::HttpMethodStub, RequestStub}, response::ResponseStub};

    use super::*;

    #[actix_rt::test]
    async fn should_verify_simple() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().await }));
        let mut app = init_service(app).await;
        let stub = JsonStub {
            id: None,
            uuid: None,
            priority: None,
            request: RequestStub { method: HttpMethodStub::from("GET"), ..Default::default() },
            response: ResponseStub { status: Some(200), ..Default::default() },
        };
        ActixApp(&mut app).verify(stub, OsString::from("simple")).await;
    }
}