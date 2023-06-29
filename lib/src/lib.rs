//!
//! This crate proposes a reimplementation of [Wiremock](http://wiremock.org/).
//! Its aims at converting [Wiremock stubs](http://wiremock.org/docs/stubbing/) into
//! [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) mocks.
//!
//! You can also use [stubr-build](https://crates.io/crates/stubr-build) to share stubs between a producer project and a consumer one.
//!
//! Also available as a [cli](https://crates.io/crates/stubr-cli).
//!
//! # use it
//!
//! ```no_run
//! use isahc;
//! use stubr::*;
//! use asserhttp::*;
//!
//! #[async_std::test]
//! async fn simple_async() {
//!     // supply a directory containing json stubs. Invalid files are just ignored
//!     let stubr = Stubr::start("tests/stubs").await;
//!     // or just mount a single file
//!     let stubr = Stubr::start("tests/stubs/hello.json").await;
//!     // or configure it (more configurations to come)
//!     let stubr = Stubr::start_with("tests/stubs", Config { port: Some(8080), ..Default::default() }).await;
//!     isahc::get_async(stubr.uri()).await.expect_status_ok();
//! }
//!
//! #[test]
//! fn simple_blocking() {
//!     // can also be used in a blocking way
//!     let stubr = Stubr::start_blocking("tests/stubs");
//!     let stubr = Stubr::start_blocking_with("tests/stubs", Config { port: Some(8080), ..Default::default() });
//!     isahc::get(stubr.uri()).expect_status_ok();
//! }
//! ```
//!
//! # macro
//!
//! ```no_run
//! use isahc;
//! use stubr::*;
//! use asserhttp::*;
//!
//! #[async_std::test]
//! #[stubr::mock] // <- takes all stubs under "tests/stubs"
//! async fn with_macro() {
//!     surf::get(stubr.uri()).await.expect_status_ok();
//! }
//!
//! #[async_std::test]
//! #[stubr::mock("pets", port = 4321)] // <- takes all stubs under "tests/stubs/pets"
//! async fn with_path_and_port() {
//!     surf::get(stubr.uri()).await.expect_status_ok();
//! }
//! ```
//!
//! # configuration
//!
//! A [`Stubr`] server can be configured globally thanks to [`Config`] struct.
//!
//! ```
//! use stubr::Config;
//! let config = Config {
//!     // server port, defaults to random
//!     port: Some(8080),
//!     // enable verbose logs
//!     verbose: true,
//!     // global delay in milliseconds. Supersedes any locally defined one.
//!     global_delay: Some(2000),
//!     // delay in milliseconds added to any locally defined one. Simulates network latencies.
//!     latency: Some(2000),
//!     // Enables verification via https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect
//!     verify: true,
//! };
//! ```
//!
//! [`Config`]: Config
//! [`Stubr`]: Stubr

#[macro_use]
extern crate lazy_static;

#[cfg(all(feature = "record-standalone", feature = "record-isahc"))]
pub use record::client::isahc_client;
#[cfg(all(feature = "record-standalone", feature = "record-reqwest"))]
pub use record::client::reqwest_client;
#[cfg(any(feature = "record-actix", feature = "record-standalone"))]
pub use record::config::RecordConfig;
#[cfg(feature = "record-standalone")]
pub use record::core::Record;
#[cfg(feature = "record-actix")]
pub use record::record_client::actix::{ActixRecord, ActixRecordMiddleware};
#[cfg(feature = "record-standalone")]
pub use record::standalone::StubrRecord;
pub use server::{config::Config, Stubr};
#[cfg(all(feature = "attributes", feature = "iso"))]
pub use stubr_attributes::iso;
#[cfg(all(feature = "record-standalone", feature = "attributes"))]
pub use stubr_attributes::record;
#[cfg(all(feature = "attributes", feature = "wiremock"))]
pub use stubr_attributes::wiremock;
#[cfg(feature = "attributes")]
pub use stubr_attributes::{apps, mock};
#[cfg(feature = "verify-actix")]
pub use verify::actix::lifecycle::ActixVerifyLifecycle;
#[cfg(feature = "verify")]
pub use verify::{StubrVerify, VerifyExcept};
#[cfg(feature = "wiremock")]
pub use wiremock_java::{WiremockExt, WiremockImage};

pub use error::{StubrError, StubrResult};

mod cloud;
pub mod error;
mod gen;
mod model;
#[cfg(any(
    feature = "record-standalone",
    feature = "record-actix",
    feature = "record-isahc",
    feature = "record-reqwest"
))]
mod record;
mod server;
#[cfg(feature = "verify")]
mod verify;
#[cfg(feature = "wiremock")]
mod wiremock_java;
mod wiremock_rs;
