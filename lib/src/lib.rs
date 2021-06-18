//!
//! This crate proposes a reimplementation of [Wiremock](http://wiremock.org/).
//! Its aims at converting [Wiremock stubs](http://wiremock.org/docs/stubbing/) into
//! [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) mocks.
//!
//! # use it
//!
//! ```no_run
//! use stubr::{Stubr, Config};
//! use surf;
//!
//! #[async_std::main]
//! async fn main() {
//!     // supply a directory containing json stubs. Invalid files are just ignored
//!     let srv = Stubr::start("tests/stubs").await;
//!     // or just mount a single file
//!     let srv = Stubr::start("tests/stubs/ping.json").await;
//!     // or configure it
//!     let srv = Stubr::start_with("tests/stubs", Config { port: Some(8080), ..Default::default() }).await;
//!     // can also be used in a blocking way
//!     let srv = Stubr::start_blocking("tests/stubs");
//!     let srv = Stubr::start_blocking_with("tests/stubs", Config { port: Some(8080), ..Default::default() });
//!
//!     // use '.uri()' method to get server address
//!     surf::get(srv.uri()).await;
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
//!     verbose: Some(true),
//!     // global delay in milliseconds. Supersedes any locally defined one.
//!     global_delay: Some(2000),
//!     // delay in milliseconds added to any locally defined one. Simulates network latencies.
//!     latency: Some(2000),
//! };
//! ```
//!
//! [`Config`]: Config
//! [`Stubr`]: Stubr

#[macro_use]
extern crate lazy_static;

pub use record::{config::RecordConfig, StubrRecord};
#[cfg(feature = "test-isahc")]
pub use record::test::isahc_client;
pub use server::{config::Config, Stubr};

pub use stubr_attributes::mock;

mod model;
mod server;
mod cloud;
mod record;