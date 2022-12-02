# Benchmark

## vs wiremock
A very simple benchmark comparing [stubr](https://github.com/beltram/stubr) to wiremock is
available [here](https://github.com/beltram/stubr/blob/main/bench/README.md)

## standalone

A benchmark of [stubr](https://github.com/beltram/stubr) itself, powered by [criterion](https://crates.io/crates/criterion) is available for each release.
The latest is available [here](https://github.com/beltram/stubr/releases/latest/download/bench.tar.gz). It aims at
tracking down progresses/regressions made.

I'm still looking for a way to turn this into something more ergonomic, especially I'd like to provide a way to compare
2 benchmarks. Meanwhile, you can download the latest benchmark with these commands.

```bash
mkdir stubr-bench &&
curl -L https://github.com/beltram/stubr/releases/latest/download/bench.tar.gz | tar xz - -C stubr-bench
```

Then open `./stubr-bench/report/index.html` in your browser.