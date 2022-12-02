# Cli

You can use [stubr](https://github.com/beltram/stubr) as a cli for serving Wiremock stubs on a local server or as proxy for recording http traffic into
json stubs.  

To get a list of all available options run `stubr --help`

| arg          |                                                       about                                                        |                                examples                                 |
|--------------|:------------------------------------------------------------------------------------------------------------------:|:-----------------------------------------------------------------------:|
| `[dir]`      |                                    Folder containing stubs or individual stub.                                     |       `stubr ./my-app-stubs` or `stubr ./my-app-stubs/ping.json`        |
| `--root-dir` | Directory containing a `mappings` folder with all stubs. Equivalent to Wiremock's one. Has precedence over `[dir]` |                    `stubr --root-dir ./my-app-stubs`                    |
| `--port`     |                                       Server port. Defaults to random port.                                        |                 `stubr --port 8080` or `stubr -p 8080`                  |
| `--delay`    |                 Global delay duration applied to all stubs (supersedes any locally defined delay).                 |         `stubr --delay 2s` or `stubr -d 1m` or `stubr -d 100ms`         |
| `--latency`  |                        Delay added to any locally defined delay. Simulates network latency.                        |        `stubr --latency 2s` or `stubr -l 1m` or `stubr -l 100ms`        |
| `completion` |                                Generates & installs bash or zsh completion scripts                                 |            `stubr completion bash` or `stubr completion zsh`            |
| `--help`     |                                                   Displays help.                                                   | `stubr help` or `stubr -h` for short help. `stubr --help` for long help |
| `--version`  |                                             Displays `stubr` version.                                              |                     `stubr -V` or `stubr --version`                     |

## install it
### precompiled binaries

If you don't want to install [Rust toolchain](https://rustup.rs/), you can always download precompiled binaries. They
have the advantage of being optimized with [upx](https://upx.github.io/) hence they are just smaller than the ones you'd
get from sources.

#### linux:
```bash
curl -L https://github.com/beltram/stubr/releases/latest/download/stubr-linux.tar.gz | tar xz - -C /usr/local/bin
```

#### macos:
```bash
curl -L https://github.com/beltram/stubr/releases/latest/download/stubr-macos.tar.gz | tar xz - -C /usr/local/bin
```

### from source
```bash
cargo install stubr-cli
```

### once installed, generate completion

Completion files generation is currently supported for `bash` and `zsh`. [Stubr](https://github.com/beltram/stubr) cli
provides a `completion` command to generate and install them in a standard location.

```bash
stubr completion zsh
# or
stubr completion bash
```

## getting started

The simplest usage is for serving Wiremock stubs under a directory (or just a single file).  
For example let's generate a simple stub file.

```bash
echo "{\"request\": {\"method\": \"GET\"}, \"response\": { \"status\": 200 }}" > hello.json
```

Then simply run it with the following command.

```bash
stubr hello.json
```

Which will generate something like that.

```bash
 > + mounted "hello.json"
 > Started stubr in 50ms on http://127.0.0.1:49604
```