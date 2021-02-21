/*!
This crate proposes a reimplementation of [Wiremock](http://wiremock.org/).
Its aims at converting [Wiremock stubs](http://wiremock.org/docs/stubbing/) into
[wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) mocks.
*/
#[macro_use]
extern crate lazy_static;

pub use server::{config::Config, Stubr, traits::AnyStubServer};

mod model;
mod server;