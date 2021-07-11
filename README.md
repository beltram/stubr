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

# assets

* [as a crate](https://crates.io/crates/stubr)
* [as a cli](https://crates.io/crates/stubr-cli)
* [Docker](#Docker)
* [Helm chart](#Helm)
* [IDE completion](#ide-completion)

# features

We list here all the capabilities supported by the original [Wiremock](https://github.com/tomakehurst/wiremock) and we
track down which ones are implemented by stubr. For having a view of what is supported in a json stub refer to the
[Wiremock cheat sheet](https://github.com/beltram/stubr/tree/main/lib#wiremock-cheat-sheet).

*Items marked with (\*) are exclusive to stubr*  

<details open>
<summary><b>Global</b></summary>

* [ ] anything related to xml
* [x] start server on custom port
* [x] (*) OpenTracing support ([OpenZipkin B3 propagation](https://github.com/openzipkin/b3-propagation))

</details>

<details>
<summary><b><a href="http://wiremock.org/docs/request-matching/"> Request matching</a></b></summary>

* [x] body
  * [x] `equalToJson`
  * [x] `ignoreExtraElements`
  * [x] `ignoreArrayOrder`
  * [x] `matchesJsonPath`
  * [x] `binaryEqualTo`
  * [x] `expression`
  * [x] `contains`
* [x] method (GET, POST, ANY etc...)
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
* [x] basic auth

</details>

<details>
<summary><b><a href="http://wiremock.org/docs/stubbing/"> Stubbing</a></b></summary>

* [x] `status`
* [x] `headers`
* [x] `bodyFileName`
* [x] `jsonBody`
* [x] `body`

</details>

<details>
<summary><b><a href="http://wiremock.org/docs/simulating-faults/"> Simulating faults</a></b></summary>

* [x] `fixedDelayMilliseconds`
* [x] global delay
* [x] (*) latency (global delay added to local ones)
* [ ] random delay

</details>

<details>
<summary><b><a href="http://wiremock.org/docs/response-templating/"> Response templating</a></b></summary>

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
* [x] String helpers
* [x] Number helpers
* [ ] assignment helpers
* [ ] XPath helpers
* [x] jsonPath helper
* [x] date and time helpers
* [ ] Random value helper
* [ ] Pick random helper
* [x] String trim helper
* [x] Base64 helper
* [x] URL encoding helper
* [ ] Form helper
* [ ] Regular expression extract helper
* [ ] Size helper
* [ ] Hostname helper
* [ ] System property helper

</details>

# Docker

A docker image is published [here](https://github.com/users/beltram/packages/container/package/stubr) with each release.  

You can play with it with the following commands:

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
server: stubr(0.4.7)

Hello stubr
```

# Helm

A Helm chart is also maintained for those moments where you have to deploy mocks in a Kubernetes cluster.

You can play with it with the following commands:

*Pending Helm `--include-dir` flag the workaround is to unpack the chart then copy stubs folder inside it.*

```bash
mkdir stubs &&
echo "{\"request\": {\"method\": \"GET\"}, \"response\": { \"body\": \"Hello stubr\" }}" > stubs/hello.json &&
helm pull --repo https://beltram.github.io/stubr/ stubr --untar &&
mv stubs stubr &&
helm install hello-stubr ./stubr
```

For the hurry ones who also have [k3d](https://k3d.io/) installed locally you can bootstrap a Kubernetes cluster
locally and install stubr chart on it with the following command
```bash
curl https://raw.githubusercontent.com/beltram/stubr/main/charts/example/run.sh | sh
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



# IDE completion

A json schema is also maintained [here](schemas/stubr.schema.json) to provide completion in IDE. It just contains completion
for features implemented in stubr and should alleviate you from a bit of pain when writing json from scratch.

<details open>
<summary><b>IntelliJ Ultimate</b></summary>

*Manual installation is required pending the schema is added to [schemastore](https://github.com/SchemaStore/schemastore)*

* Go to `Settings > Languages & Frameworks > Schemas & DTDs > JSON Schema Mappings`
* Add a mapping (click on the upper `+`)
* Then supply the following
  * name: `stubr`
  * Schema file or URL: `https://raw.githubusercontent.com/beltram/stubr/main/schemas/stubr.schema.json`
  * Schema version: `JSON Schema version 7`
  * File path pattern: `stubs/*.json` (and `mappings/*.json` if you want to use it for original wiremock stubs)
* The `Apply`

</details>