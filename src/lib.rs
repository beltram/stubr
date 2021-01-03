/*!
This crate proposes a reimplementation of [Wiremock](http://wiremock.org/).
Its aims at converting [Wiremock stubs](http://wiremock.org/docs/stubbing/) into
[wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) mocks.
*/
pub use server::{Stubr, traits::AnyStubServer, config::Config};

mod model;
mod server;