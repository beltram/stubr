# stubr

[![Build status](https://github.com/beltram/stubr/workflows/ci/badge.svg)](https://github.com/beltram/stubr/actions)
[![Crates.io](https://img.shields.io/crates/v/stubr.svg)](https://crates.io/crates/stubr)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/stubr)

Extends [wiremock-rs](https://crates.io/crates/wiremock) by supporting
[Wiremock](https://github.com/tomakehurst/wiremock) json stubs as input.

Use it when you have to mock an external service over http and want a language agnostic format for representing your
mocks. Especially shines when the service you have to mock already proposes and
publishes [Wiremock](https://github.com/tomakehurst/wiremock)
e.g. [Spring Boot](https://spring.io/projects/spring-boot)
with [Spring Cloud Contract](https://spring.io/projects/spring-cloud-contract).

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
let srv = Stubr::start_with("tests/stubs", Config { port: Some(8080), ..Default::default () }).await;
// can also be used in a blocking way
let srv = Stubr::start_blocking("tests/stubs");
let srv = Stubr::start_blocking_with("tests/stubs", Config { port: Some(8080), ..Default::default () });

// use '.uri()' method to get server address
surf::get(srv.uri()).await;
```

# wiremock cheat sheet

This is a condensed reminder of Wiremock documentation regarding json stubs format. It is also a view of the currently
implemented features in `stubr` : just things which actually work in `stubr` are present.

*You can also get assistance for writing json stubs
with [IDE completion](https://github.com/beltram/stubr#ide-completion) provided by stubr.*

```json
{
  "id": "82d86e05-9ee0-44ca-9a8d-1fc6f719437e", // (optional) unique stub identifier. Returned in 'Matched-Stub-Id' header
  "request": {
    "method": "GET", // (optional) http method. Can be "ANY" to match any method. Defaults to "ANY"
    "urlPath": "/api/exact-url", // exact uri match
    "urlPathPattern": "/api/regex-url/([a-z]{4})", // uri must match regex
    "urlPattern": "/api/regex-url/([a-z]{4})\\?and=([a-z]{4})", // uri & query must match regex
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
    "basicAuth" : { // exact Basic authentication matching
      "username" : "user",
      "password" : "pass"
    },
    "bodyPatterns": [
      { "equalToJson": {"name": "bob"} }, // strict json request body equality
      { "equalToJson": {"name": "bob"}, "ignoreExtraElements": true }, // ignore extra json fields supplied in request body. Default to false.
      { "equalToJson": {"name": "bob"}, "ignoreArrayOrder": true }, // ignore array items order. Default to false.
      { "matchesJsonPath": "$.name" }, // must just match json path
      { "matchesJsonPath": "$.consoles[?(@.name == 'xbox')]" }, // must match json path + equality
      { "matchesJsonPath": "$.consoles[?(@.price > 200)]" }, // must match json path + bound
      { "expression": "$.name", "contains": "o" }, // must match json path + contain the letter 'o'
      { "binaryEqualTo": "AQID" /* Base 64 */ } // byte array equality
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
      "Content-Type": "application/pdf" // returns this response header
    },
    // ..now response templating
    // it uses handlebars and allows you to define dynamic response based upon the content of the request
    // it can be used in "jsonBody", "body", "bodyFileName" or "headers"
    "transformers": ["response-template"], // required to activate response templating
    "jsonBody": {
      "url-path-and-query": "{{request.url}}",
      "url-path": "{{request.path}}",
      "url-path-segments": "{{request.pathSegments.[1]}}", // returns 'two' given '/one/two/three' path
      "query": "{{request.query.kind}}", // returns 'comics' given '/api/books?kind=comics'
      "multi-query": "{{request.query.kind.[1]}}", // returns 'novel' given '/api/books?kind=comics&kind=novel'
      "method": "{{request.method}}", // http request method e.g. "POST"
      "header": "{{request.headers.Content-Type}}", // returns request header with given key
      "multi-header": "{{request.headers.cache-control.[0]}}", // returns first value of "cache-control" values
      "body": "{{request.body}}", // returns raw request body
      "from-request": "{{jsonPath request.body '$.name'}}", // takes field 'name' from json request body
      "now": "{{now}}", // current datetime (UTC)
      "now-fmt": "{{now format='yyyy/MM/dd'}}", // (1) with custom Java SimpleDateFormat
      "now-fmt-epoch": "{{now format='epoch'}}", // epoch in milliseconds
      "now-fmt-unix": "{{now format='unix'}}", // epoch in seconds
      "now-positive-offset": "{{now offset='3 days'}}", // human time positive offset
      "now-negative-offset": "{{now offset='-3 days'}}", // human time negative offset
      "now-with-timezone": "{{now timezone='Europe/Rome'}}",
      "number-is-odd": "{{isOdd 3}}", // or 'isEven'
      "number-stripes": "{{stripes request.body 'if-even' 'if-odd'}}",
      "string-trim": "{{trim request.body}}", // removes leading & trailing whitespaces
      "base64-encode": "{{base64 request.body padding=false}}", // padding is optional and defaults to true
      "base64-decode": "{{base64 request.body decode=true}}",
      "url-encode": "{{urlEncode request.header.x-raw}}",
      "url-decode": "{{urlEncode request.header.x-encoded decode=true}}"
    }
  }
}
```

* (1) [Java SimpleDateFormat](https://docs.oracle.com/javase/7/docs/api/java/text/SimpleDateFormat.html)

# recording

Stubr can be used to record http traffic in your unit tests and dump them into json stubs.
Currently, integration is quite limited but much more (actix, warp, rocket, tide) are around the corner.  

You need to start a tokio multi-threaded runtime in your unit test so as the recorder starts. Use
`#[tokio::test(flavor = "multi_thread")]` for that.  

The recorder acts as a proxy, so you need to configure your http client to use this proxy. Currently, thanks to the
`test-isahc` feature you can get a configured [isahc](https://github.com/sagebind/isahc) client with
`Stubr::record().isahc_client()`. Your stubs will then be stored under `target/stubs/localhost`

```rust
use stubr::Stubr;
use isahc;

#[tokio::test(flavor = "multi_thread")] // required for recording
async fn sample_test() {
    // start a standalone http server to record, for example stubr itself
    let srv = Stubr::start("tests/stubs");
    Stubr::record().isahc_client().get(srv.uri()).unwrap();
    // stubs will be created under `target/stubs`
}
```