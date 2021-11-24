<h1 align="center">stubr</h1>
<div align="center">
 <strong>
   Wiremock reimplemented in Rust
 </strong>
</div>
<br />
<div align="center">
  <a href="https://docs.rs/stubr">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://github.com/beltram/stubr/actions">
    <img src="https://github.com/beltram/stubr/workflows/ci/badge.svg?style=flat-square"
      alt="ci" />
  </a>
  <a href="https://coveralls.io/github/beltram/stubr?branch=main">
    <img src="https://coveralls.io/repos/github/beltram/stubr/badge.svg?branch=main" alt="coverage" />
  </a>
</div>
<br/>

Extends [wiremock-rs](https://crates.io/crates/wiremock) by supporting
[Wiremock](https://github.com/tomakehurst/wiremock) json stubs as input.

Use it when you have to mock an external service over http and want a language agnostic format for representing your
mocks. Especially shines when the service you have to mock already proposes and
publishes [Wiremock](https://github.com/tomakehurst/wiremock)
e.g. [Spring Boot](https://spring.io/projects/spring-boot)
with [Spring Cloud Contract](https://spring.io/projects/spring-cloud-contract).

You can use [stubr-build](https://crates.io/crates/stubr-build) to share stubs between a producer project and a consumer
one.

Also available as a [cli](https://crates.io/crates/stubr-cli).

# use it

First prepare some stubs

```bash
echo "{\"request\": {\"method\": \"GET\"}, \"response\": { \"status\": 200 }}" > tests/stubs/hello.json
```

Then use this stub in your tests.

```rust
use isahc;
use stubr::*;
use asserhttp::*;

#[async_std::test]
#[stubr::mock] // <- you can also provide stubs path here e.g. #[stubr::mock("hello.json")]
async fn with_macro() {
    surf::get(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock]
async fn simple_async() {
    // supply a directory containing json stubs. Invalid files are just ignored
    let stubr = Stubr::start("tests/stubs").await;
    // or just mount a single file
    let stubr = Stubr::start("tests/stubs/hello.json").await;
    // or configure it (more configurations to come)
    let stubr = Stubr::start_with("tests/stubs", Config { port: Some(8080), ..Default::default () }).await;
    isahc::get_async(stubr.uri()).await.expect_status_ok();
}

#[test]
#[stubr::mock]
fn simple_blocking() {
    // can also be used in a blocking way
    let stubr = Stubr::start_blocking("tests/stubs");
    let stubr = Stubr::start_blocking_with("tests/stubs", Config { port: Some(8080), ..Default::default () });
    isahc::get(stubr.uri()).expect_status_ok();
}
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
      "string-capitalized": "{{capitalize mister}}", // or 'decapitalize'
      "string-uppercase": "{{upper mister}}", // or 'lower'
      "number-stripes": "{{stripes request.body 'if-even' 'if-odd'}}",
      "string-trim": "{{trim request.body}}", // removes leading & trailing whitespaces
      "size": "{{size request.body}}", // string length or array length
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

Stubr can be used to record http traffic in your unit tests and dump them into json stubs. Currently, integration is
quite limited but much more (actix, warp, rocket, tide) are around the corner.

The recorder acts as a standalone proxy server, so you need to configure your http client to use it.  
You can use the `record-isahc` feature to get a configured [isahc](https://github.com/sagebind/isahc) client with
`Stubr::record().isahc_client()` or the `record-reqwest` feature to get a configured
[reqwest](https://github.com/seanmonstar/reqwest) client with `Stubr::record().reqwest_client()`. Your stubs will then
be stored under `target/stubs/localhost`

```rust
use stubr::Stubr;
use isahc;

// this requires `record` and `record-reqwest` (or `record-isahc`) features which are not default.

#[tokio::test(flavor = "multi_thread")] // required for recording
#[stubr::mock] // start a standalone http server to record, for example stubr itself
async fn sample_test() {
    Stubr::record().reqwest_client().get(stubr.uri()).send().await.unwrap();
    Stubr::record().isahc_client().get(stubr.uri()).unwrap();
    // stubs will be created under `target/stubs`
}
```