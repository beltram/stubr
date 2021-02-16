# stubr

[![Build status](https://github.com/beltram/stubr/workflows/ci/badge.svg)](https://github.com/beltram/stubr/actions)
[![Crates.io](https://img.shields.io/crates/v/stubr.svg)](https://crates.io/crates/stubr)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/stubr)
[![Packaging status](https://repology.org/badge/tiny-repos/stubr.svg)](https://repology.org/project/stubr/badges)

# use it

```rust
use stubr::Stubr;

let srv = Stubr::start("tests/stubs").await;
// or just mount a single file
let srv = Stubr::start("tests/stubs/ping.json").await;
// or configure it (more configurations to come)
let srv = Stubr::start_with("tests/stubs", Config { port: Some(8080) }).await;

// use '.uri()' method to get server address
surf::get( & srv.uri()).await;
```