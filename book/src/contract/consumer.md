# As a consumer

We will now consume the stubs we have verified on the [producer](producer.md) side. It actually does not change from 
when you mount regular stubs, you will just use different helpers here.

## importing

So we have some stubs defined in a Rust project, let's call it `actix-producer`. On naive way to deal with this would 
be to simply copy/paste stubs from producer to consumer. That'd work, obviously ; but we would be too cumbersome to
maintain and way too error-prone. We need a more automated way to import those stubs.  

Here comes [stubr-build](https://crates.io/crates/stubr-build). It's a simple build dependency which will scan your
build dependencies and look for producers (projects with a root `stubs` folder with json stubs underneath). For each, it
will copy/paste those stubs under `target/stubr/{consumer-name}/{producer-name}`. This default location will be used 
later on to mount the stubs.  

To begin with, add [stubr-build](https://crates.io/crates/stubr-build) to your build dependencies. Then also add the
producers (here we will use `actix-producer`).

```toml
[build-dependencies]
stubr-build = "0.6.2"
actix-producer = "0.1.0"
```

Then, in a build script, invoke [stubr-build](https://crates.io/crates/stubr-build) and do `cargo build`

```rust,ignore,noplayground,edition2021
{{#include ../../../actix-consumer/build.rs}}
```

Verify your stubs have been imported (given your consumer project is called `actix-consumer`), you should have 
something like this under `target/stubr`:

```text
├── actix-consumer
│ └── actix-producer
│   ├── beer-create-conflict-name.json
│   ├── beer-create.json
│   ├── beer-find-by-id-not-found.json
│   └── beer-find-by-id.json
```

## consuming

At this point, your consumer app could be anything: either another actix application, or a web application using a
different framework, or a simple cli or batch relying on a http client to call your producer. It does not matter. Here,
we'll assume the simplest use case (a simple blocking http client using reqwest) but it does not make any difference.  

We will use the `apps` attribute macro to mount the stubs we just imported in our stub server. And for our tests. we
will just import the stubs of the `actix-producer` app we created previously. To do so, add 
`#[stubr::apps("actix-producer")]` on your test method (note that you can use it to mount many apps e.g. 
`#[stubr::apps("svc-a", "svc-b")]`). This will create a local binding with the name of your app.

```rust,ignore,noplayground,edition2021
{{#include ../../../actix-consumer/tests/api/beer.rs:sample_apps_binding}}
```

You can then use this binding to get the uri(s) of the mock server(s) and execute your tests against it.

```rust,ignore,noplayground,edition2021
{{#include ../../../actix-consumer/tests/api/beer.rs:contract_consumer_test}}
```

And that's all folks ! We have consumed our producer stubs. But those stubs only have hardcoded values and will be hard 
to change and maintain. We'll now see how to relax them.
