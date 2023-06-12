# Stubs

Writing stubs can be challenging and time-consuming. [Stubr](https://github.com/beltram/stubr) tries to assist by
providing [IDE completion](../ide-completion.md) or by [recording](../recording/index.md) live traffic into stubs. But
you still have to know how to write a stub and what helpers you have in order to [relax](../contract/relaxing.md) your stubs as much
as possible.  

You will find here in a single snippet **ALL** the fields/helpers available to you:

```json
{
  "id": "82d86e05-9ee0-44ca-9a8d-1fc6f719437e", // (optional) unique stub identifier. Returned in 'Matched-Stub-Id' header
  "priority": 1, // (optional) helps solving interlaced conditions (many stubs match the request). 1 is the highest priority, 255 the lowest
  "request": {
    "method": "GET", // (optional) http method. Can be "ANY" to match any method. Defaults to "ANY"
    "urlPath": "/api/exact-uri", // exact URI match
    "urlPathPattern": "/api/regex-uri/([a-z]{4})", // URI must match regex
    "urlPattern": "/api/regex-uri/([a-z]{4})\\?and=([a-z]{4})", // URI & query must match regex
    "url": "/api/uri?age=young", // raw URI + query parameters by equality matching
    "queryParameters": {
      "firstname": { "equalTo": "beltram" }, // by equality matching (can also be an int, or a boolean)
      "lastname": { "equalTo": "maldant", "caseInsensitive": true }, // case insensitve equality
      "age": { "absent": true }, // must be absent
      "city": { "contains": "a" }, // must contain the letter 'a'
      "title": { "matches": "([A-Za-z]+)" }, // must match regex
      "job": { "doesNotMatch": "([A-Za-z]+)" }, // or must not match regex
    },
    "headers": {
      "content-type": { "equalTo": "application/json" } // by equality matching
      // .. then all matchers described above for query parameters are also applicable here
    },
    "basicAuth" : { // exact Basic authentication matching
      "username": "user",
      "password": "pass"
    },
    "jwtAuth": {
      "equalTo": "eyJhbGciOiJSUzI1NiJ9.e30.MBkQ...", // plain JWT token
      "alg": {
        "equalTo": "RS256", // JWT algorithm by equality matcher
        "oneOf": ["RS256", "HS256"] // JWT must contain one of these algorithms
      },
      "payloadPatterns": [
        // all matchers available in 'bodyPatterns' ⬇️
      ]
    },
    "bodyPatterns": [
      { "equalToJson": {"name": "bob"} }, // strict json request body equality
      { "equalToJson": {"name": "bob"}, "ignoreExtraElements": true }, // ignore extra json fields supplied in request body. Default to false.
      { "equalToJson": {"name": "bob"}, "ignoreArrayOrder": true }, // ignore array items order. Default to false.
      { "matchesJsonPath": "$.name" }, // must just match json path
      { "matchesJsonPath": "$.consoles[?(@.name == 'xbox')]" }, // must match json path + equality
      { "matchesJsonPath": "$.consoles[?(@.price > 200)]" }, // must match json path + bound
      { "expression": "$.name", "contains": "at" }, // must match json path + contain the string 'at'
      { "expression": "$.user", "equalToJson": { "name": "bob" } }, // must match json path + be equal
      { "binaryEqualTo": "AQID" /* Base 64 */ } // byte array equality
    ]
  },
  "response": {
    "status": 200, // (required) response status
    "fixedDelayMilliseconds": 2000, // delays response by 2 seconds
    "delayDistribution": { // a random delay..
      "type": "lognormal", // ..with logarithmic distribution
      "median": 100, // The 50th percentile of latencies in milliseconds
      "sigma": 0.1 // Standard deviation. The larger the value, the longer the tail
    },
    "jsonBody": { // json response body (automatically adds 'content-type:application/json' header)
      "name": "john",
      "surnames": [ "jdoe", "johnny" ]
    },
    "body": "Hello World !", // text response (automatically adds 'Content-Type:text/plain' header)
    "base64Body": "AQID", // binary Base 64 body
    "bodyFileName": "tests/stubs/response.json", // path to a .json or .txt file containing the response
    "headers": {
      "content-type": "application/pdf" // returns this response header
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
      "now-fmt": "{{now format='yyyy/MM/dd'}}", // with custom Java SimpleDateFormat
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
      "url-decode": "{{urlEncode request.header.x-encoded decode=true}}",
      // you can also use 'any*' helpers. They will produce a random value
      "regex": "{{anyRegex '[a-z]{4}'}}", // generate a random string matching regex
      "string": "{{anyNonEmptyString}}", // or '{{anyNonEmptyString}}'
      "alphanum": "{{anyAlphaNumeric}}",
      "boolean": "{{anyBoolean}}",
      "uuid": "{{anyUuid}}",
      "ip": "{{anyIpAddress}}", // e.g. '127.0.0.1'
      "host": "{{anyHostname}}", // e.g. 'https://github.com'
      "email": "{{anyEmail}}", // e.g. 'john.doe@gmail.com'
      "enum": "{{anyOf 'alpha' 'beta' 'gamma'}}", // returns randomly one of those 3 values
      "number": "{{anyNumber}}", // integer or float 
      "integer": "{{anyI32}}", // also all Rust int types (u32, u8, i64 etc..)
      "float": "{{anyFloat}}",
      "anyDate": "{{anyDate}}" // or 'anyTime', 'anyDatetime', 'anyIso8601'
    }
  }
}
```