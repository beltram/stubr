use std::future::{ready, Future, Ready};
use std::pin::Pin;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};

pub struct ActixVerifyLifecycle<T>(pub fn(&T));

impl<S, B, T> Transform<S, ServiceRequest> for ActixVerifyLifecycle<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ActixVerifyLifecycleMiddleware<S, T>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ActixVerifyLifecycleMiddleware {
            service,
            before_each: self.0,
        }))
    }
}

pub struct ActixVerifyLifecycleMiddleware<S, T> {
    service: S,
    before_each: fn(&T),
}

impl<S, B, T> Service<ServiceRequest> for ActixVerifyLifecycleMiddleware<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (http_req, payload) = req.into_parts();
        if let Some(data) = http_req.app_data::<Data<T>>() {
            (self.before_each)(data);
        }
        let fut = self.service.call(ServiceRequest::from_parts(http_req, payload));
        Box::pin(fut)
    }
}
