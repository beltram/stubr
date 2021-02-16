# stubr

[![LICENSE](https://img.shields.io/badge/license-Apache_2-blue.svg)](LICENSE)
[![Build status](https://github.com/beltram/stubr/workflows/ci/badge.svg)](https://github.com/beltram/stubr/actions)
[![Crates.io](https://img.shields.io/crates/v/stubr.svg)](https://crates.io/crates/stubr)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/stubr)
[![Packaging status](https://repology.org/badge/tiny-repos/stubr.svg)](https://repology.org/project/stubr/badges)

Adaptation of [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) supporting existing
[Wiremock](https://github.com/tomakehurst/wiremock) json stubs as input.  
Aims at reaching feature parity with [Wiremock](https://github.com/tomakehurst/wiremock) and be a drop-in replacement of
the latter.

# usage

 * [as a crate](lib/README.md)
 * [as a cli](cli/README.md)
 * Docker image (incoming)
 * Helm chart (incoming)

# features

 * [x] request matching (json)
   * [x] body
     * [x] `equalToJson`
     * [x] `matchesJsonPath`
     * [ ] `binaryEqualTo`
   * [x] method (GET, POST etc...)
   * [x] url
     * [x] `url`
     * [x] `urlPath`
     * [x] `urlPathPattern`
     * [ ] `urlPattern`
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
     * [ ] templating
 * [ ] anything related to xml
 * [ ] config
   * [x] custom port
   * [ ] global delay