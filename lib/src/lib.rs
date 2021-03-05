/*!
* This crate proposes a reimplementation of [Wiremock](http://wiremock.org/).
* Its aims at converting [Wiremock stubs](http://wiremock.org/docs/stubbing/) into
* [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) mocks.
*
* # use it
*
* ```no_run
* use stubr::{Stubr, Config};
* use surf;
*
* // supply a directory containing json stubs. Invalid files are just ignored
* let srv = Stubr::start("tests/stubs").await;
* // or just mount a single file
* let srv = Stubr::start("tests/stubs/ping.json").await;
* // or configure it (more configurations to come)
* let srv = Stubr::start_with("tests/stubs", Config { port: Some(8080), ..Default::default() }).await;
*
* // use '.uri()' method to get server address
* surf::get(srv.uri()).await;
* ```
*/
#[macro_use]
extern crate lazy_static;

pub use server::{config::Config, Stubr, traits::AnyStubServer};

mod model;
mod server;