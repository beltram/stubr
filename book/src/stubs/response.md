# Response

In the response part, you have to define the actual http response the stub server will serve when the stub matches the
incoming http request as we defined it with [request matching](../stubs/request.md)

## Status

The http response status. In the range `[100..599]`.

```json
{
  "response": {
    "status": 200
  }
}
```

## Header

Http response headers. Note that keys are case-insensitive. Multivalued headers are not supported yet. You can
use [response templating](#response-templating) here as well if you add `"transformers": ["response-template"]`.

```json
{
  "response": {
    "transformers": [
      "response-template"
    ],
    "headers": {
      "content-type": "application/json",
      "ETag": "33a64df551425fcc55e4d42a148795d9f25f89d4",
      "location": "{{request.url}}/1234"
    }
  }
}
```

## Body

There are different ways to define a http response. We'll just focus here on supplying hardcoded values in the response,
but you can relax all those fields with templates. We'll see that immediately in the next chapter.

```json
{
  "response": {
    "body": "Hello World !",
    "base64Body": "AQID",
    "bodyFileName": "tests/stubs/response.json",
    "jsonBody": {
      "name": "john",
      "surnames": [
        "jdoe",
        "johnny"
      ]
    }
  }
}
```

* `body` use this one if you have a text body or anything simple. If the body is large you'd better opt
  for `bodyFileName`.
* `base64Body` if the body is not utf-8 encoded use it to supply a body as byte. Those have to be base 64 encoded.
* `bodyFileName` when the response gets large or to factorize some very common bodies, it is sometimes preferable to
  extract it in a file. When using it in a Rust project, the file path is relative to the workspace root. You can also
  use templating to dynamically select a file.
* `jsonBody` when the body is json. Even though such a body can be defined with all the previous fields, it is more
  convenient to define a json response body here.

## Relaxed field

Using only hardcoded values is a good way to start mocking things. But as time goes on, your project might start to get
bloated with a lot of stubs. You will also see the limit of hardcoded values when
doing [contract testing](../contract/index.md).

In order to "relax" your stub, you will have to use [Handlebars](https://handlebarsjs.com/) helpers. They will allow you
to have random values generated for you, because, most of the time, that's what the actual application does. And, as a
consumer, you also don't care about the actual value of this field in your test i.e. `"age": "{{anyU8}}"` will work in
all your unit tests because none of your unit tests expects a particular value for this field.

In order to use a Handlebars helper, you need to add `"transformers": ["response-template"]`.

Keep in mind that such helper will also be used to generate assertions when you will be using this stub
for [contract testing](../contract/index.md) while [verifying your producer](../contract/producer.md).

NB: those templates are not available in [Wiremock](https://github.com/tomakehurst/wiremock), you can only use them in
[stubr](https://github.com/beltram/stubr).

```json
{
  "response": {
    "transformers": [
      "response-template"
    ],
    "jsonBody": {
      "regex": "{{anyRegex '[a-z]{4}'}}",
      "string": "{{anyNonEmptyString}}",
      "alphanum": "{{anyAlphaNumeric}}",
      "boolean": "{{anyBoolean}}",
      "uuid": "{{anyUuid}}",
      "ip": "{{anyIpAddress}}",
      "host": "{{anyHostname}}",
      "email": "{{anyEmail}}",
      "enum": "{{anyOf 'alpha' 'beta' 'gamma'}}",
      "number": "{{anyNumber}}",
      "integer": "{{anyI32}}",
      "float": "{{anyFloat}}",
      "anyDate": "{{anyDate}}"
    }
  }
}
```

* `anyRegex` generates a value matching this regex. Tip: most of the time will be used for strings but if this regex
  defines an integer, a float or a boolean and is used in `"jsonBody""` the generated value will be cast
* `anyNonEmptyString` or `anyNonBlankString` generates an arbitrary utf-8 string
* `anyAlphaNumeric` generates an arbitrary string with only alphanumeric characters
* `anyBoolean` generates either `true` or `false`
* `anyUuid` generates a random UUIDv4
* `anyIpAddress` generates a random IP address e.g. `127.0.0.1`
* `anyHostname` generates an arbitrary hostname e.g. `https://github.com`
* `anyEmail` generates a random valid email address e.g. `john.doe@gmail.com`
* `anyOf` given the supplied values, will pick one randomly. Only works for strings.
* `anyNumber` when one does not care about the number size, generates either an integer or a float
* `anyI32` or `anyU32` etc.. generates a random integer. Possible values
  are: `anyU64`, `anyI64`, `anyU32`, `anyI32`, `anyU16`, `anyI16`, `anyU8`, `anyI8`
* `anyFloat` generates a random float
* `anyDate` generates a date with format `yyyy-mm-dd`
* `anyTime` generates a time with format `hh:mm:ss`
* `anyDatetime` generates a datetime with format `yyyy-mm-ddThh:mm:ss`
* `anyIso8601` generates an [iso-8601](https://en.wikipedia.org/wiki/ISO_8601) compliant datetime

## Response templating

Another kind of relaxing you can do is by being able to represent as best as possible the actual http response of your
app. Very often, a field in the response is the exact same as the one in the request e.g. in a POST request to create
a REST resource. You can use in your response parts of the request to do so.

```json
{
  "response": {
    "transformers": [
      "response-template"
    ],
    "jsonBody": {
      "url-path-and-query": "{{request.url}}",
      "url-path": "{{request.path}}",
      "url-path-segments": "{{request.pathSegments.[1]}}",
      "query": "{{request.query.kind}}",
      "multi-query": "{{request.query.kind.[1]}}",
      "method": "{{request.method}}",
      "header": "{{request.headers.Content-Type}}",
      "multi-header": "{{request.headers.cache-control.[0]}}",
      "body": "{{request.body}}",
      "from-request": "{{jsonPath request.body '$.name'}}"
    }
  }
}
```

* `request.url` given a request to `http://localhost/api/path?a=b` returns `path?a=b`
* `request.path` given a request to `http://localhost/api/path?a=b` returns `api/path`
* `request.pathSegments.[i]` allows picking a part of the url path (`i` is zero indexed) e.g.
  `http://localhost/a/b/c` with `i` == 1 returns `b`
* `query.<selector>.[i]` allows picking a named query parameter. Replace `<selector>` by the name of the query
  parameter. If the query parameter is multivalued, you can select only one with the zero indexed `i`. For example with
  `http://localhost?a=1&a=2&a=3&b=1` then `{{query.b}}` returns `1` and `{{query.a.[1]}}` returns `2`
* `request.method` returns the (uppercase) http request method. If you want the lowercase method
  just `{{lower request.method}}`
* `request.headers.<selector>.[i]` about the same as picking query parameters. Note that here `selector` is
  case-insensitive.
* `request.body` takes the raw request body without altering it
* `jsonPath request.body '<json-path>'` for templating only a field from request's json body. `json-path` is the
  JSONPath
  for selecting the right field. Use an [online JSONPath evaluator](https://jsonpath.com/) to try out your paths.

You also sometimes have to generate dynamic data or to transform existing one:

```json
{
  "response": {
    "transformers": [
      "response-template"
    ],
    "jsonBody": {
      "now": "{{now}}",
      "now-fmt": "{{now format='yyyy/MM/dd'}}",
      "now-fmt-epoch": "{{now format='epoch'}}",
      "now-fmt-unix": "{{now format='unix'}}",
      "now-positive-offset": "{{now offset='3 days'}}",
      "now-negative-offset": "{{now offset='-3 days'}}",
      "now-with-timezone": "{{now timezone='Europe/Rome'}}",
      "number-is-odd": "{{isOdd request.body}}",
      "number-stripes": "{{stripes request.body 'if-even' 'if-odd'}}",
      "string-capitalized": "{{capitalize request.body}}",
      "string-uppercase": "{{upper request.body}}",
      "string-trim": "{{trim request.body}}",
      "size": "{{size request.body}}",
      "base64-encode": "{{base64 request.body padding=false}}",
      "base64-decode": "{{base64 request.body decode=true}}",
      "url-encode": "{{urlEncode request.header.x-raw}}",
      "url-decode": "{{urlEncode request.header.x-encoded decode=true}}"
    }
  }
}
```

* `now` by default return the current datetime in [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339) format (this is
  only for backward compatibility with Wiremock)
    * `format` could be either:
        * a custom [Java SimpleDateFormat](https://docs.oracle.com/javase/7/docs/api/java/text/SimpleDateFormat.html) (
          for Wiremock compatibility) e.g. `format='yyyy/MM/dd'`
        * `epoch` Unix timestamp in milliseconds
        * `unix` Unix timestamp in seconds
    * `offset` now with the given offset expressed in human-readable format. Refer
      to [humantime documentation](https://docs.rs/humantime/latest/humantime/fn.parse_duration.html) for further
      examples.
    * `timezone` for using a string timezone (
      see [list](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List))
* `isOdd` or `isEven` returns a boolean whether the numeric value is an even or odd integer
* `capitalize` first letter to uppercase e.g. `mister` becomes `Mister`
* `upper` or `lower` recapitalizes the whole word
* `stripes` returns alternate values depending if the tested value is even or odd
* `trim` removes leading & trailing whitespaces
* `size` returns the number of bytes for a string (⚠️ not the number of characters) or the size of an array
* `base64` for standard (no base64 url encoding yet) Base64 encoding
    * `decode` for decoding when true
    * `padding` with/without padding
* `urlEncode` for url encoding the value. Use `decode=true` to decode

## Simulate fault

You can also use [stubr](https://github.com/beltram/stubr) to simulate http server runtime behaviour. And most of the
time you'll want to introduce
latencies
to check how your consuming application reacts to such delays. Currently, the options are quite sparse but should grow !

```json
{
  "expect": 2,
  "response": {
    "fixedDelayMilliseconds": 2000
  },
  "delayDistribution": {
    // a random delay with logarithmic distribution
    "type": "lognormal",
    "median": 100,
    // The 50th percentile of latencies in milliseconds
    "sigma": 0.1
    // Standard deviation. The larger the value, the longer the tail
  }
}
```

* `expect` will allow to verify that your unit test has not called the given stub more than N times. Turn it on like
  this `stubr::Stubr::start_with(stubr::Config { verify: true, ..Default::default() })`
  or `#[stubr::mock(verify = true)]` with the attribute macro
* `fixedDelayMilliseconds` a delay (in milliseconds) added everytime this stub is matched. If you are
  using [stubr](https://github.com/beltram/stubr)
  standalone through the [cli](../cli.md), this value can be either superseded by `--delay` or complemented
  by `--latency`
* `delayDistribution` for random delays (always in milliseconds), use `type` to choose the one
    * `lognormal` is a pretty good approximation of long tailed latencies centered on the 50th
      percentile. [Try different values](https://www.wolframalpha.com/input/?i=lognormaldistribution%28log%2890%29%2C+0.4%29)
      to find a good approximation.
        * `median`: the 50th percentile of latencies in milliseconds
        * `sigma`: standard deviation. The larger the value, the longer the tail.