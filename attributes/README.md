<h1 align="center">stubr-attributes</h1>
<div align="center">
 <strong>
   Macros for stubr
 </strong>
</div>
<br />
<div align="center">
  <a href="https://docs.rs/stubr-attributes">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://github.com/beltram/stubr/actions">
    <img src="https://github.com/beltram/stubr/workflows/ci/badge.svg?style=flat-square"
      alt="ci" />
  </a>
  <a href="https://coveralls.io/github/beltram/stubr?branch=main">
    <img src="https://coveralls.io/repos/github/beltram/stubr/badge.svg?branch=main" alt="coverage" />
  </a>
</div>
<br/>

## #[stubr::mock]

Starts a Stubr mock server and creates a `stubr` variable which can be used to call the server e.g. `stubr.uri()`.
It supports both standard and async test functions.

```rust
use isahc;
use asserhttp::*; // optional

#[test]
#[stubr::mock] // <- takes stubs under crate's "tests/stubs" by default
fn simple_test() {
    isahc::get(stubr.uri()).expect_status_ok();
}
```

## #[stubr::record]

Can also be used for recording with `#[stubr::record]`. It will spawn a standalone proxy which will record all http exchanges.  
A `recorder` variable is created so that you can interact with the proxy.
You then need to configure your http client to use this proxy. With `record-isahc` or `record-reqwest` features you can get
an http client configured to hit the recorder proxy.
It supports both standard and async test functions.

```rust
use isahc;
use asserhttp::*; // optional

#[stubr::mock]
#[stubr::record] // <- takes stubs under crate's "tests/stubs" by default
#[test]
fn simple_test() {
    recorder.isahc_client().get(stubr.uri()).expect_status_ok();
    // a recorded stub has been created under 'target/stubs'
}
```