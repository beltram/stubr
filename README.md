<h1 align="center">stubr</h1>
<div align="center">
 <strong>
   Wiremock reimplemented in Rust
 </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/stubr">
    <img src="https://img.shields.io/crates/v/stubr.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/stubr">
    <img src="https://img.shields.io/crates/d/stubr.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/stubr">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- license -->
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache_2-blue.svg?style=flat-square"
      alt="Apache 2" />
  </a>
  <!-- CI status -->
  <a href="https://github.com/beltram/stubr/actions">
    <img src="https://github.com/beltram/stubr/workflows/ci/badge.svg?style=flat-square"
      alt="ci" />
  </a>
  <!-- Code coverage -->
  <a href="https://coveralls.io/github/beltram/stubr?branch=main">
    <img src="https://coveralls.io/repos/github/beltram/stubr/badge.svg?branch=main" alt="coverage" />
  </a>
</div>
<br/>

Adaptation of [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) supporting existing
[Wiremock](https://github.com/tomakehurst/wiremock) json stubs as input.  
Aims at reaching feature parity with [Wiremock](https://github.com/tomakehurst/wiremock) and be a drop-in replacement of
the latter.

# usage

* [as a crate](lib/README.md)
* [as a cli](cli/README.md)
* [Docker](#Docker)
* Helm chart (incoming)

# features

* [x] request matching (json)
    * [x] body
        * [x] `equalToJson`
        * [x] `matchesJsonPath`
        * [x] `binaryEqualTo`
    * [x] method (GET, POST etc...)
    * [x] url
        * [x] `url`
        * [x] `urlPath`
        * [x] `urlPathPattern`
        * [x] `urlPattern`
    * [x] headers
        * [x] `equalTo`
        * [x] `contains`
        * [x] `matches`
        * [x] `caseInsensitive`
        * [x] `absent`
        * [ ] multivalued
    * [x] query parameters
        * [x] `equalTo`
        * [x] `contains`
        * [x] `matches`
        * [x] `caseInsensitive`
        * [x] `absent`
* [x] response
    * [x] `status`
    * [x] `headers`
    * [x] `bodyFileName`
    * [x] `jsonBody`
    * [x] `body`
    * [x] `fixedDelayMilliseconds`
    * [x] templating
        * [x] `{{request.url}}`
        * [x] `{{request.path}}`
        * [x] `{{request.pathSegments.[<n>]}}`
        * [x] `{{request.query.<key>}}`
        * [x] `{{request.query.<key>.[<n>]}}`
        * [x] `{{request.method}}`
        * [ ] `{{request.host}}`
        * [ ] `{{request.scheme}}`
        * [ ] `{{request.baseUrl}}`
        * [x] `{{request.headers.<key>}}`
        * [x] `{{request.headers.<key>.[<n>]}}`
        * [ ] `{{request.cookies.<key>}}`
        * [ ] `{{request.cookies.<key>.[<n>]}}`
        * [x] `{{request.body}}`
        * [ ] Handlebars helpers
        * [ ] Number and assignment helpers
        * [ ] XPath helpers
        * [x] jsonPath helper
        * [ ] date and time helpers
        * [ ] Random value helper
        * [ ] Pick random helper
        * [ ] String trim helper
        * [ ] Base64 helper
        * [ ] URL encoding helper
        * [ ] Form helper
        * [ ] Regular expression extract helper
        * [ ] Size helper
        * [ ] Hostname helper
        * [ ] System property helper
* [ ] anything related to xml
* [x] config
    * [x] custom port
    * [x] global delay
    * [ ] random delay

# Docker

A docker image is published [here](https://github.com/users/beltram/packages/container/package/stubr) with each release.  

You can play with it with the following commands

```bash
echo "{\"request\": {\"method\": \"GET\"}, \"response\": { \"body\": \"Hello stubr\" }}" > hello.json &&
docker run -v $(pwd):/stubs -d --rm -p 8080:8080 ghcr.io/beltram/stubr:latest /stubs -p 8080 &&
http :8080
```

Which should output

```bash
HTTP/1.1 200 OK
content-length: 11
content-type: text/plain
date: Tue, 23 Mar 2021 13:37:41 GMT
server: stubr(0.3.4)

Hello stubr
```

# benchmark

Performance matters for `stubr` because it is meant to be a lighter version of [Wiremock](https://github.com/tomakehurst/wiremock).

### comparing to wiremock

A very simple benchmark comparing `stubr` to wiremock is
available [here](https://github.com/beltram/stubr/blob/main/bench/report.md).  

### cargo bench

A benchmark of `stubr` itself, powered by [criterion](https://crates.io/crates/criterion) is available for each release.
The latest is available [here](https://github.com/beltram/stubr/releases/latest/download/bench.tar.gz).
It aims at tracking down progresses/regressions made.  

I'm still looking for a way to turn this into something more ergonomic, especially I'd like to provide a way to compare
2 benchmarks. Meanwhile, you can download the latest benchmark with these commands.

```bash
mkdir stubr-bench &&
curl -L https://github.com/beltram/stubr/releases/latest/download/bench.tar.gz | tar xz - -C stubr-bench
```

Then open `./stubr-bench/report/index.html` in your browser.