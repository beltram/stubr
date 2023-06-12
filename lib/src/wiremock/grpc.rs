use crate::wiremock::mock_set::MountedMockState;
use crate::wiremock::{mock_server::bare_server::MockServerState, mock_set::MountedMockSet, ResponseTemplate};
use futures_timer::Delay;
use hyper::{Body, Request};
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) async fn handle_grpc(
    request: Request<Body>, server_state: Arc<RwLock<MockServerState>>,
) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
    let wiremock_request = crate::wiremock::Request::from_hyper(request).await;
    let (response, delay) = server_state.write().await.handle_grpc_request(wiremock_request).await;

    if let Some(delay) = delay {
        delay.await;
    }

    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(response)
}

impl MockServerState {
    pub(crate) async fn handle_grpc_request(
        &mut self, request: crate::wiremock::Request,
    ) -> (hyper::Response<hyper::Body>, Option<futures_timer::Delay>) {
        self.mock_set.handle_grpc_request(request).await
    }
}

impl MountedMockSet {
    pub(crate) async fn handle_grpc_request(&mut self, request: crate::wiremock::Request) -> (hyper::Response<hyper::Body>, Option<Delay>) {
        let mut response_template: Option<ResponseTemplate> = None;
        self.mocks.sort_by_key(|(m, _)| m.specification.priority);
        for (mock, mock_state) in &mut self.mocks {
            if *mock_state == MountedMockState::OutOfScope {
                continue;
            }
            if mock.matches(&request) {
                response_template = mock.response_template(&request).ok();
                break;
            }
        }
        if let Some(response_template) = response_template {
            let delay = response_template.delay().map(|d| Delay::new(d.into_owned()));
            (response_template.generate_grpc_response(), delay)
        } else {
            let default_resp = tonic::codegen::http::Response::builder()
                .status(200)
                .header::<_, i32>("grpc-status", tonic::Code::NotFound.into())
                .header("content-type", "application/grpc")
                .body(hyper::Body::from(vec![0u8; 5]))
                .unwrap();
            (default_resp, None)
        }
    }
}

impl ResponseTemplate {
    /// Start building a `ResponseTemplate` specifying the status code of the response.
    pub fn new_grpc(s: impl Into<tonic::Code>) -> Self {
        Self {
            http_status_code: None,
            grpc_status_code: Some(s.into()),
            headers: std::collections::HashMap::new(),
            mime: None,
            body: None,
            delay: None,
            lognormal_delay: None,
        }
    }

    /// Generate a response from the template.
    pub(crate) fn generate_grpc_response(&self) -> hyper::Response<hyper::Body> {
        let body = self.body.clone().unwrap_or_else(|| vec![0u8; 5]);
        let body = hyper::Body::from(body);
        let code: i32 = self.grpc_status_code.unwrap().into();
        tonic::codegen::http::Response::builder()
            .status(200)
            .header("grpc-status", code.to_string())
            .header("content-type", "application/grpc")
            .body(body)
            .unwrap()
    }
}
