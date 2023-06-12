//! Forking the excellent [wiremock](https://crates.io/crates/wiremock) since this crates will
//! diverge a lot from it: gRPC support, focus more on standalone mode and using it in Docker
//! meaning no panic allowed
mod delay;
#[cfg(feature = "grpc")]
pub mod grpc;
pub mod http;
pub mod matchers;
mod mock;
mod mock_server;
mod mock_set;
mod mounted_mock;
mod request;
mod respond;
mod response_template;
mod verification;

pub use mock::{Match, Mock, MockBuilder, Times};
pub use mock_server::{MockGuard, MockServer, MockServerBuilder};
pub use request::Request;
pub use respond::Respond;
pub use response_template::ResponseTemplate;
