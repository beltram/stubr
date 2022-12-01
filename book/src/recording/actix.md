# Recording your actix app

You can plug recording in your existing [actix integration tests](https://actix.rs/docs/testing/#integration-tests) by 
just adding a single line: `.wrap(stubr::ActixRecord::default())`. This will register a middleware which will capture
the http request and response, then dump them under `./target/stubs/localhost`.  

*This requires the `record-actix` feature.*

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/actix.rs}}
```