# stubr

[![Build status](https://github.com/beltram/stubr/workflows/ci/badge.svg)](https://github.com/beltram/stubr/actions)
[![Crates.io](https://img.shields.io/crates/v/stubr.svg)](https://crates.io/crates/stubr)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/stubr)
[![Packaging status](https://repology.org/badge/tiny-repos/stubr.svg)](https://repology.org/project/stubr/badges)

Extends [wiremock-rs](https://crates.io/crates/wiremock) by supporting
[Wiremock](https://github.com/tomakehurst/wiremock) json stubs as input.  

Use it when you have to mock an external service over http and want a language agnostic format for representing
your mocks. Especially shines when the service you have to mock already proposes and publishes [Wiremock](https://github.com/tomakehurst/wiremock)
e.g. [Spring Boot](https://spring.io/projects/spring-boot) with [Spring Cloud Contract](https://spring.io/projects/spring-cloud-contract). 

# use it

```rust
use stubr::Stubr;

let srv = Stubr::start("tests/stubs").await;
// or just mount a single file
let srv = Stubr::start("tests/stubs/ping.json").await;
// or configure it (more configurations to come)
let srv = Stubr::start_with("tests/stubs", Config { port: Some(8080) }).await;

// use '.uri()' method to get server address
surf::get(&srv.uri()).await;
```