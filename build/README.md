<h1 align="center">stubr-build</h1>
<div align="center">
 <strong>
   Build plugin for consuming stubs
 </strong>
</div>
<br />
<div align="center">
  <a href="https://docs.rs/stubr-build">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://coveralls.io/github/beltram/stubr?branch=main">
    <img src="https://coveralls.io/repos/github/beltram/stubr/badge.svg?branch=main" alt="coverage" />
  </a>
</div>
<br/>

One of the key benefit of relying upon json files for stubbing your web applications is that they are both portable and
language agnostic.  
This crates aims at introducing a 'producer-driven' way of testing your applications: a producer (which exposes the API)
publishes its mocks ; consumer(s) app(s) pull them and can use those stubs in their tests, configuring their http
clients to hit them.

## as a producer

Currently, you just have to place your stubs in a 'stubs' directory at crate root, something like this:

```bash
├── src
├── stubs
    ├── get.json
    └── post.json
└── Cargo.toml
```

If you have some `include` or `exclude` in your `Cargo.toml` make sure it does not involve the `stubs` directory.

## as a consumer

In your `Cargo.toml` use `stubr-build` in your `build-dependencies` and also add your producer apps. You also need to
invoke it in a `build.rs` file.

```toml
[package]
build = "build.rs"

[build-dependencies]
stubr-build = "0.5.0-rc.1"
producer-a = "<version>"
producer-b = "<version>"
```

Then in your `build.rs`:

```rust
fn main() { stubr_build::stubr_consumer() }
```

In order to extract stubs (invoke the build script), `cargo build` has to be invoked. So think about executing it before
tests in your CI or locally.

To mount those stubs in a server you can then use a macro

```rust
#[test]
#[stubr::apps("producer-a", "producer-b")]
fn my_test() {
    // local bindings with the name of each producer are created
    isahc::get(producer_a.uri()).expect_status_ok();
    isahc::get(producer_b.uri()).expect_status_ok();
}
```

Or without macros

```rust
#[test]
fn my_test() {
    let apps = Stubr::apps_blocking(&["producer-a", "producer-b"]);
    let (producer_a, producer_b) = (apps.get(0).unwrap(), apps.get(1).unwrap());
    isahc::get(producer_a.uri()).expect_status_ok();
    isahc::get(producer_b.uri()).expect_status_ok();
}
```