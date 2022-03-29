use std::fmt::Debug;

use actix_http::Request as ActixRequest;
use actix_service::{IntoServiceFactory, Service as ActixService, ServiceFactory};
use actix_web::{dev::{AppConfig, ServiceResponse}, test::TestRequest};
use async_trait::async_trait;

use super::{
    mapping::{req::StdRequest, resp::{RequestAndStub, StdResponse}},
    stub_finder::ProducerStubFinder,
    StubrVerify,
    VerifyExcept,
};

mod req;
mod resp;
pub mod lifecycle;

#[async_trait(? Send)]
impl<A, T> StubrVerify<T> for A where
    A: IntoServiceFactory<T, ActixRequest>,
    T: ServiceFactory<ActixRequest, Config=AppConfig, Response=ServiceResponse>,
    <T as ServiceFactory<ActixRequest>>::InitError: Debug,
{
    async fn verify_except<N>(self, except: impl VerifyExcept<N> + 'async_trait) {
        let srv = self.into_factory();
        if let Ok(app) = srv.new_service(AppConfig::default()).await {
            for (stub, name) in ProducerStubFinder::find_stubs(except) {
                let req = StdRequest::try_from(&stub).unwrap_or_else(|_| panic!("Could not verify '{:?}'. Invalid json stub.", name));
                let test_req = TestRequest::from(&req).set_payload(Vec::<u8>::from(&stub.request)).to_request();
                let resp: StdResponse = app.call(test_req).await
                    .unwrap_or_else(|_| panic!("Failed verifying stub {:?}", name))
                    .into();
                RequestAndStub { req, stub: stub.response, name }.verify(resp);
            };
        }
    }
}