# Recording your isahc calls

The only way currently to record isahc is to spawn a proxy and configure isahc to use this proxy. This is exactly what
the following snippet does.  

*This requires the `record-isahc` feature.*

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/isahc_standalone.rs}}
```

You can find your recorded stubs under `./target/stubs/localhost`