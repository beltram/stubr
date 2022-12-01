# Recording standalone

If you don't fall into one of the previous boxes (for example if you use another http client), you can still record the http
traffic by using the standalone recording proxy, the exact same one used in the [cli](../cli.md).  

To do so, you just have to spawn the proxy and then configure your http client to use this proxy.

*This requires the `record` feature.*

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/standalone.rs}}
```

Or, in order to keep the syntax short, you can use the provided attribute macro.

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/standalone_macro.rs}}
```