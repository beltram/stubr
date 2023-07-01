use std::{net::TcpListener, sync::Arc};

use hyper::{
    http,
    service::{make_service_fn, service_fn},
};

use crate::{error::HyperError, wiremock_rs::mock_server::bare_server::MockServerState};

type HyperResult<T> = Result<T, HyperError>;

/// The actual HTTP server responding to incoming requests according to the specified mocks.
pub(super) async fn try_run_server(
    listener: TcpListener, server_state: Arc<tokio::sync::RwLock<MockServerState>>, shutdown_signal: tokio::sync::oneshot::Receiver<()>,
) -> HyperResult<()> {
    let request_handler = make_service_fn(move |_| {
        let server_state = server_state.clone();
        async move {
            let svc = service_fn(move |request: hyper::Request<hyper::Body>| {
                let server_state = server_state.clone();
                async move {
                    let content_type = request.headers().get("content-type").map(|v| v.as_bytes());
                    match content_type {
                        Some(b"application/grpc") => {
                            #[cfg(feature = "grpc")]
                            {
                                Ok(crate::wiremock_rs::grpc::handle_grpc(request, server_state).await?)
                            }
                            #[cfg(not(feature = "grpc"))]
                            {
                                HyperResult::Err(HyperError::ImplementationError)
                            }
                        },
                        _ => handle_http(request, server_state).await,
                    }
                }
            });
            HyperResult::Ok(svc)
        }
    });

    let server = hyper::Server::from_tcp(listener)?
        .executor(LocalExec)
        .serve(request_handler)
        .with_graceful_shutdown(async {
            // This futures resolves when either:
            // - the sender half of the channel gets dropped (i.e. MockServer is dropped)
            // - the sender is used, therefore sending a poison pill willingly as a shutdown signal
            let _ = shutdown_signal.await;
        });

    Ok(server.await?)
}

async fn handle_http(
    request: hyper::Request<hyper::Body>, server_state: Arc<tokio::sync::RwLock<MockServerState>>,
) -> HyperResult<hyper::Response<hyper::Body>> {
    let wiremock_request = crate::wiremock_rs::Request::from_hyper(request).await;
    let (response, delay) = server_state.write().await.handle_request(wiremock_request).await;

    // We do not wait for the delay within the handler otherwise we would be
    // holding on to the write-side of the `tokio::sync::RwLock` on `mock_set`.
    // Holding on the lock while waiting prevents us from handling other requests until
    // we have waited the whole duration specified in the delay.
    // In particular, we cannot perform even perform read-only operation -
    // e.g. check that mock assumptions have been verified.
    // Using long delays in tests without handling the delay as we are doing here
    // caused tests to hang (see https://github.com/seanmonstar/reqwest/issues/1147)
    if let Some(delay) = delay {
        delay.await;
    }

    http_types_response_to_hyper_response(response).await
}

// An executor that can spawn !Send futures.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalExec;

impl<F> hyper::rt::Executor<F> for LocalExec
where
    F: std::future::Future + 'static, // not requiring `Send`
{
    fn execute(&self, fut: F) {
        // This will spawn into the currently running `LocalSet`.
        tokio::task::spawn_local(fut);
    }
}

pub(crate) async fn http_types_response_to_hyper_response(mut response: http_types::Response) -> HyperResult<hyper::Response<hyper::Body>> {
    let version = response.version().map(|v| v.into()).unwrap_or_default();
    let mut builder = http::response::Builder::new().status(response.status() as u16).version(version);

    let hyperium_headers = builder.headers_mut().ok_or(HyperError::ImplementationError);
    try_headers_to_hyperium_headers(response.as_mut(), hyperium_headers?)?;

    let body = response
        .take_body()
        .into_bytes()
        .await
        .map_err(|_| HyperError::HttpTypesError)?;
    Ok(builder.body(body.into())?)
}

fn try_headers_to_hyperium_headers(headers: &mut http_types::Headers, hyperium_headers: &mut http::HeaderMap) -> HyperResult<()> {
    for (name, values) in headers {
        let name = http::header::HeaderName::from_bytes(name.as_str().as_bytes())?;

        for value in values.iter() {
            let value = http::header::HeaderValue::from_bytes(value.as_str().as_bytes())?;
            hyperium_headers.append(&name, value);
        }
    }
    Ok(())
}
