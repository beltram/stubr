# stubr

[![Build status](https://github.com/beltram/stubr/workflows/ci/badge.svg)](https://github.com/beltram/stubr/actions)
[![Crates.io](https://img.shields.io/crates/v/stubr.svg)](https://crates.io/crates/stubr)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/stubr)

Extends [wiremock-rs](https://crates.io/crates/wiremock) by supporting
[Wiremock](https://github.com/tomakehurst/wiremock) json stubs as input.  

Use it when you have to mock an external service over http and want a language agnostic format for representing
your mocks. Especially shines when the service you have to mock already proposes and publishes [Wiremock](https://github.com/tomakehurst/wiremock)
e.g. [Spring Boot](https://spring.io/projects/spring-boot) with [Spring Cloud Contract](https://spring.io/projects/spring-cloud-contract).  

Also available as a [cli](https://crates.io/crates/stubr-cli).

# use it

```rust
use stubr::{Stubr, Config};
use surf;

// supply a directory containing json stubs. Invalid files are just ignored
let srv = Stubr::start("tests/stubs").await;
// or just mount a single file
let srv = Stubr::start("tests/stubs/ping.json").await;
// or configure it (more configurations to come)
let srv = Stubr::start_with("tests/stubs", Config { port: Some(8080), ..Default::default() }).await;
// can also be used in a blocking way
let srv = Stubr::start_blocking("tests/stubs");
let srv = Stubr::start_blocking_with("tests/stubs", Config { port: Some(8080), ..Default::default() });

// use '.uri()' method to get server address
surf::get(srv.uri()).await;
```

# wiremock cheat sheet

This is a condensed reminder of Wiremock documentation regarding json stubs format. It is also a view of the currently
implemented features in `stubr` : just things which actually work in `stubr` are present.  

```json
{
  "id": "82d86e05-9ee0-44ca-9a8d-1fc6f719437e", // (optional) unique stub identifier. Returned in 'Matched-Stub-Id' header
  "request": {
    "method": "GET", // (required) http method. Can be "ANY" to match any method
    "urlPath": "/api/exact-url", // exact uri match
    "urlPathPattern": "/api/regex-url/([a-z]{4})", // uri must match regex
    "url": "/api/url?age=young", // raw url + query parameters by equality matching
    "queryParameters": {
      "firstname": { "equalTo": "beltram" }, // by equality matching (can also be an int, or a boolean)
      "lastname": { "equalTo": "maldant", "caseInsensitive": true }, // case insensitve equality
      "age": { "absent": true } // must be absent
      "city": { "contains": "a" } // must contain the letter 'a'
      "title": { "matches": "([A-Za-z]+)" } // must match regex
      "job": { "doesNotMatch": "([A-Za-z]+)" } // or must not match regex
    },
    "headers": {
      "Content-Type": { "equalTo": "application/json" } // by equality matching
      // .. then all matchers described above for query parameters are also applicable here
    },
    "bodyPatterns": [
      { "equalToJson": {"name": "bob"} }, // strict json request body equality
      { "matchesJsonPath": "$.name" }, // must just match json path
      { "matchesJsonPath": "$.consoles[?(@.name == 'xbox')]" }, // must match json path + equality
      { "matchesJsonPath": "$.consoles[?(@.price > 200)]" }, // must match json path + bound
      { "expression": "$.name", "contains": "o" } // must match json path + contain the letter 'o'
    ]
  },
  "response": {
    "status": 200, // (required) response status
    "fixedDelayMilliseconds": 2000, // delays response by 2 seconds
    "jsonBody": { // json response (automatically adds 'Content-Type:application/json' header)
      "name": "john",
      "surnames": [ "jdoe", "johnny" ]
    },
    "body": "Hello World !", // text response (automatically adds 'Content-Type:text/plain' header)
    "bodyFileName": "tests/stubs/response.json", // path to a .json or .txt file containing the response
    "headers": {
      "Content-Type": "application/pdf" // return this response header
    },
    // ..now response templating
    // it uses handlebars and allows you to define dynamic response based upon the content of the request
    // it can be used in "jsonBody", "body", "bodyFileName" or "headers"
    "jsonBody": {
      "url-path-and-query": "{{request.url}}",
      "url-path": "{{request.path}}",
      "url-path-segments": "{{request.pathSegments.[1]}}", // will return 'two' given '/one/two/three' path
      "query": "{{request.query.kind}}", // will return 'comics' given '/api/books?kind=comics'
      "multi-query": "{{request.query.kind.[1]}}", // will return 'novel' given '/api/books?kind=comics&kind=novel'
      "method": "{{request.method}}", // http request method e.g. "POST"
      "header": "{{request.headers.Content-Type}}", // will return request header with given key
      "body": "{{request.body}}" // will return raw request body
    }
  }
}
```