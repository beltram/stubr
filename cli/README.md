# stubr

[![Build status](https://github.com/beltram/stubr/workflows/ci/badge.svg)](https://github.com/beltram/stubr/actions)
[![Crates.io](https://img.shields.io/crates/v/stubr.svg)](https://crates.io/crates/stubr-cli)

# use it

You can use `stubr` as a cli for serving Wiremock stubs on a local server.  
To get a list of all available options run `stubr --help`

The simplest usage is for serving Wiremock stubs under a directory.  
Example for a project exposing contracts using Spring Cloud Contract

```bash
./gradlew generateClientStubs
stubr build/stubs/META-INF/com.ecorp/my-app/SNAPSHOT/mappings
 > + mounted "./build/stubs/META-INF/com.ecorp/my-app/SNAPSHOT/mappings/find-all.json"
 > + mounted "./build/stubs/META-INF/com.ecorp/my-app/SNAPSHOT/mappings/find-by-id.json"
 > Started stubr in 50ms on http://127.0.0.1:49604
```

## args/flags/options

| arg | about | examples |
|-----|:-----:|:-------:|
| `[dir]` | Folder containing stubs or individual stub. | `stubr ./my-app-stubs` or `stubr ./my-app-stubs/ping.json` |
| `--root-dir` | Directory containing a `mappings` folder with all stubs. Equivalent to Wiremock's one. Has precedence over `[dir]` | `stubr --root-dir ./my-app-stubs` |
| `--port` | Server port. Defaults to random port. | `stubr --port 8080` or `stubr -p 8080` |
| `--delay` | Global delay duration applied to all stubs (supersedes any locally defined delay). | `stubr --delay 2s` or `stubr -d 1m` or `stubr -d 100ms` |
| `--latency` | Delay added to any locally defined delay. Simulates network latency. | `stubr --latency 2s` or `stubr -l 1m` or `stubr -l 100ms` |
| `completion` | Generates & installs bash or zsh completion scripts | `stubr completion bash` or `stubr completion zsh` |
| `--help` | Displays help. | `stubr help` or `stubr -h` for short help. `stubr --help` for long help |
| `--version` | Displays `stubr` version. | `stubr -V` or `stubr --version` |

Also available as a [crate](https://crates.io/crates/stubr).

# install it

## with precompiled binaries (linux & osx)

##### linux:

```bash
curl -L https://github.com/beltram/stubr/releases/latest/download/stubr-linux.tar.gz | tar xz - -C /usr/local/bin
```

##### macos (Catalina):

**NOTE:** Big Sur users are recommended to install [with cargo](#with-cargo) or [from source](#from-source-linux--osx)

```bash
curl -L https://github.com/beltram/stubr/releases/latest/download/stubr-macos.tar.gz | tar xz - -C /usr/local/bin
```

## with cargo

```bash
cargo install stubr-cli
```

## from source (linux & osx)

```bash
git clone --depth 1 https://github.com/beltram/stubr.git && cd stubr && cargo build --release && mv target/release/stubr /usr/local/bin/
```

## then generate completion

Completion files generation is currently supported for `bash` and `zsh`. Stubr cli provides a `completion` command to
generate and install them in a standard location.

```bash
stubr completion zsh
# or
stubr completion bash
```

# benchmark

A very simple benchmark comparing stubr to wiremock is available [here](https://github.com/beltram/stubr/blob/main/bench/report.md)