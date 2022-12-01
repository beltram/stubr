# Getting started

You will of course want to use stubr in your Rust project. We will cover here the case where you want to mock an external
http application you do not own (if you own it, you might be interested in [contract testing](../contract/index.md)).  

**First you need a stub.** A stub is a json file which represents the endpoint you want to mock. You have 2 options from
now on:
* [Record](../recording/index.md) the existing application (if you are lazy)
* [Write the json stub yourself](../stubs/index.md) 🥵

We are going to be even lazier and simply create the json stub with a command.  

You should have a project layout like this:

```text
├── src
│   ├── lib.rs
└── tests
    ├── stubs
├── Cargo.toml
├── README.md
```

We are going to create the stub under `tests/stubs/`, the default location. You can place them wherever you want of course,
but you'll see it's more convenient to place them here.

```bash
echo "{\"request\": {\"method\": \"GET\", \"urlPath\": \"/hello\"}, \"response\": { \"body\": \"Hello stubr\" }}" > tests/stubs/hello.json
```

And with a few lines of code we can spawn a mock server and call (here with reqwest for example).

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/gs_1.rs}}
```

But we can further shorten this with a attribute macro: `#[stubr::mock]`

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/gs_2.rs}}
```

You can also use the macro in non-async test methods of course, the macro will adapt by itself.  
Note that here you can omit the `tests/stubs/` path prefix. If you placed your files in the default location, they are
going to be searched from there.  
As well, you can mount many stubs with the macro e.g. `#[stubr::mock("hello.json", "goodbye.json")]`.
By default, if you take care to place only stubs with different request matching under `tests/stubs`, you can simply
place `#[stubr::mock]`. It will recursively mount all the stubs under `tests/stubs`, searching also in subdirectories.  

Here are all the options you can use with the attribute macro

```rust,ignore,noplayground,edition2021
{{#include ../../../lib/tests/book/gs_3.rs}}
```

* `full_path`: use this if your stubs are not under `tests/stubs` but elsewhere. Note that it can point to a directory.
* `port` when you want an explicit port for your mock server
* `verify` to turn on verification of the number of times a stub gets called (`expect` field in your stubs). 
See [simulating fault](../stubs/response.md#simulate-fault) for reference