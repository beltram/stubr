# Recording your reqwest calls

You have 2 ways to record reqwest http calls ; either with the `stubr::Record` trait (highly recommended) or the "original"
way, still supported, with a standalone recording proxy.

*This requires the `record-reqwest` feature.*

## trait (recommended)

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/reqwest.rs}}
```

You can find your recorded stubs under `./target/stubs/localhost`

*NB: async is not supported yet*

## standalone

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/reqwest_standalone.rs}}
```

You can find your recorded stubs under `./target/stubs/localhost`
