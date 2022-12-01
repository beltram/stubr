# Request

With request matching you have to describe all the conditions the incoming http requests have to match in order for your
stub response to be served. Most of the time, you will opt in for a conservative approach where you will have exhaustive
and strict conditions. That's when you want to assess the http caller behaves the right way. Other times you do not care
about request matching at all e.g. you use stubr to benchmark a reverse proxy: in that
case `request { "method": "ANY" }` is enough. **Just write the request matching you need**.

## Method

Expects the request method. Use `ANY` when you do not care which method it will be.  
Available verbs are `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`, `TRACE`, `PATCH`

*Note: `method` is optional and defaults to `ANY`*

```json
{
  "request": {
    "method": "GET"
  }
}
```

## URI

To match request's URI (and maybe its query parameters). Only one of the following matcher is allowed. If more than one
are present, it does not fail but chooses one matcher according to the descending
priority `url` > `urlPath` > `urlPattern` > `urlPathPattern`

```json
{
  "request": {
    "url": "/api/uri?age=young",
    "urlPath": "/api/exact-uri",
    "urlPattern": "/api/regex-uri/([a-z]{4})\\?and=([a-z]{4})",
    "urlPathPattern": "/api/regex-uri/([a-z]{4})"
  }
}
```

* `url`: Matches by equality the URI and query parameters.
* `urlPath`: Matches by equality **just** the URI without query parameters.
* `urlPattern`: Matches URI and query parameters. Path segments and query parameters value can contain regexes.
* `urlPathPattern`: Matches **just** the URI without query parameters. Path segments can contain regexes.

## Query parameters

Allows matching query parameters. Prefer this instead of [URI](#uri) matching just because it is clearer. Multivalued
query parameters are not supported yet.

```json
{
  "request": {
    "queryParameters": {
      "firstname": { "equalTo": "beltram" },
      "lastname": { "equalTo": "maldant", "caseInsensitive": true },
      "age": { "absent": true },
      "birthdate": { "absent": false },
      "city": { "contains": "at" },
      "title": { "matches": "([A-Za-z]+)" },
      "job": { "doesNotMatch": "([A-Za-z]+)" }
    }
  }
}
```

* `equalTo` by equality matcher. Can be a string, a boolean, a number, null etc... Can be turned case-insensitive
  with `caseInsensitive`.
* `absent` specified query parameter key must be absent/present.
* `contains` value must contain the supplied string in a case-insensitive way
* `matches`/`doesNotMatch` value must match the supplied regex (or not)

## Headers

Header matcher are **exactly** the same as [query parameter matcher](#query-parameters) above.

```json
{
  "request": {
    "headers": {
      "content-type": { "equalTo": "application/json" }
    }
  }
}
```

## Authorization

Those matcher are exclusive to stubr and not available in Wiremock. They allow crafting more relaxed request matchers
when it comes to authorization. You could for example have stubs specialized for a specific user (we sometimes persona).

You can have matchers for Basic authentication ([RFC 7617](https://datatracker.ietf.org/doc/html/rfc7617)). For example,
for matching `Authorization: Basic am9obi5kb2U6Y2hhbmdlbWU=` you would have:

```json
{
  "request": {
    "basicAuth": {
      "username": "john.doe",
      "password": "changeme"
    }
  }
}
```

You can also match a JWT token in the `Authorization` header as
per [RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519)

```json
{
  "request": {
    "jwtAuth": {
      "equalTo": "eyJhbGciOiJSUzI1NiJ9.e30.MBkQ...",
      "alg": {
        "equalTo": "RS256",
        "oneOf": [ "RS256", "HS256" ]
      },
      "payloadPatterns": [
        { "equalToJson": { "iss": "john.doe" } },
        { "equalToJson": { "exp": 1300819380 } },
        {
          "expression": "$.address",
          "equalToJson": { "street": "rue de Rivoli", "city": "Paris" }
        }
      ]
    }
  }
}
```

* `equalTo` by equality matcher. Equivalent to `"headers":{"authorization":{"equalTo": "..."}}`. If you have this
  matcher, all the other ones will be ignored
* `alg.equalTo` by equality matcher. JWT algorithm has to be exactly this
* `alg.oneOf` JWT algorithm has to be one of the supplied values. Here are all the supported JWT
  algorithms: `HS256`, `HS384`, `HS512`, `ES256`, `ES384`, `RS256`, `RS384`, `RS512`, `PS256`, `PS384`, `PS512`, `EdDSA`
* `payloadPatterns` for matching the JWT body. Exactly the same matcher as [body](#body) ones.

## Body

```json
{
  "request": {
    "bodyPatterns": [
      { "equalToJson": { "name": "bob" } },
      {
        "equalToJson": {"names": ["alice", "bob"]},
        "ignoreExtraElements": true,
        "ignoreArrayOrder": true
      },
      { "matchesJsonPath": "$.name" },
      { "matchesJsonPath": "$.consoles[?(@.name == 'xbox')]" },
      { "matchesJsonPath": "$.consoles[?(@.price > 200)]" },
      { "expression": "$.name", "contains": "at" },
      { "expression": "$.user", "equalToJson": { "name": "bob" } },
      { "expression": "$.age", "equalToJson": 42 },
      { "binaryEqualTo": "AQID" }
    ]
  }
}
```

* `equalToJson` strict equality matcher. Request body has to be exactly equal to this. If it is not used
  with `expression`, all other matchers will be ignored. However, it can be relaxed with:
    * `ignoreExtraElements` to ignore json fields in the http request not present in the matcher
    * `ignoreArrayOrder` to match json arrays regardless the order of their items
* `expression` a [JSONPath](https://www.ietf.org/archive/id/draft-goessner-dispatch-jsonpath-00.html) matcher used to
  narrow the matching. The matched expression has then to be verified by either:
    * `equalToJson` for strict equality (can be another json object, a string, number etc..)
    * `contains` ; if json matched by `expression` is a string it must contain the supplied string
* `matchesJsonPath` json request body has to contain the supplied key identified by
  a [JSONPath](https://www.ietf.org/archive/id/draft-goessner-dispatch-jsonpath-00.html). You can also
  use [JSONPath expression](https://docs.hevodata.com/sources/streaming/rest-api/writing-jsonpath-expressions/) to also
  filter and match the json values
* `binaryEqualTo` byte equality matcher. Has to be base 64 encoded

## Priority

Sometimes, you can have 2 different stubs that could both match a given http request. This happens most of the time when
you start writing stubs for your application errors. You basically should have:

* one relaxed stub for your nominal case matching for example `"urlPathPattern": "/users/([0-9]{4})"`
* one stub for each error with hardcoded value e.g. `"urlPath": "/users/1234"` for a `404` response

The issue here is that if your stub server receives a `GET /users/1234` request, both stubs will match. You want your
error stub to have a higher than the nominal e.g. error stub will have a priority of 1 whereas the nominal one will have
a priority of 2.

```json
{
  "priority": 1
}
```

* `priority` a u8. 1 is the highest priority, 255 the lowest, 5 the default value when absent. It is optional.