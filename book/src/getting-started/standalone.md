# Getting started

You can use stubr as a standalone mock server i.e. an executable. To learn more read the pages about how to use the
[cli](../cli.md) or [as a Docker image](../docker.md).

For this short demo we are going to use the cli. We will create a http stub, mount it on a stub server and then call it
to verify it works.

## installation

#### recommended

*If you don't have it, install rustup from [here](https://rustup.rs/).*

```bash
cargo install stubr-cli
```

#### or from precompiled binaries

Those binaries are stripped with [upx](https://upx.github.io/) and then compressed. They are likely to be smaller than
the ones built by rustc which might be preferable in certain conditions

###### macos

```bash
curl -L https://github.com/beltram/stubr/releases/latest/download/stubr-macos.tar.gz | tar xz - -C /usr/local/bin
```

###### linux

```bash
curl -L https://github.com/beltram/stubr/releases/latest/download/stubr-linux.tar.gz | tar xz - -C /usr/local/bin
```

###### windows

Install binary from [here](https://github.com/beltram/stubr/releases/latest/download/stubr-windozs.zip).

## Hello world !

We are going to create the simplest stub possible. It will accept any http method on any path and will respond `200 OK`.

```bash
cat > stub.json <<- EOF
{
  "request": {
    "method": "ANY"
  },
  "response": {
    "status": 200,
    "body": "Hello world !",
  }
}
EOF
```

A few things about a stub:

* It is a json file. First because it is the format supported by [Wiremock](https://github.com/tomakehurst/wiremock) and
  we want to be compatible with it. Also, most of the time, your http APIs will consume/produce json data in their
  bodies. So you can inline the request/response body in this file without externalizing it.
* `request { .. }` is where we define **request matching** i.e. "*conditions the incoming http request has to satisfy in
  order for the response part to be served*"
* `response { .. }` is the part where you define what the stub server will respond if all request matchings pass.

#### mount it

The cli can spawn a http server with a path to the file or folder containing json stubs. By default, it will try to bind
to a random port, here we force it to attach to port `8080`.

```bash
stubr stub.json -p 8080 &
```

#### call it

Now let's verify our stub server is up and running and that it serves our stubs the right way.

```bash
curl -i http://localhost:8080
```

Which should output:

```bash
HTTP/1.1 200 OK
server: stubr(0.4.14)
content-length: 0
date: Fri, 22 Jul 2022 19:31:48 GMT

Hello world !%
```

or with httpie

```bash
http :8080
```

## Hello {name} !

Now let's spice things a bit and make our stub a bit more dynamic by capturing a part of the request path and template
it in the response (this is called [response templating](../stubs/response.md)).

But first let's kill the previous server

```bash
lsof -ti tcp:8080 | xargs kill
```

```bash
cat > hello.json <<- EOF
{
  "request": {
    "method": "GET",
    "urlPathPattern": "/hello/(.+)"
  },
  "response": {
    "status": 200,
    "body": "Hello {{request.pathSegments.[1]}} !",
    "transformers": ["response-template"]
  }
}
EOF
```

Here:

* `"urlPathPattern": "/hello/(.+)"` is one way to express URL matching. It contains a regular hardcoded path `/hello/`
  and a regular expression `(.+)` which has to match in order for the stub response to be served.
* `"transformers": ["response-template"]` will activate response templating. This allows you to inject a part of the
  request in the response. Prefer using it over hardcoded values when your real life application actually does that. The
  more you use it the better your test accuracy will be.
* `{{request.pathSegments.[1]}}` now that response templating is enabled, you can inject parts of your request in the
  response. With [Wiremock](https://github.com/tomakehurst/wiremock) as with stubr, we
  use [handlebars](https://handlebarsjs.com/) templates to do such a
  thing. [Many response templates are available](../stubs/response.md) in stubr in order to pick whatever part
  of the request you want.

#### Mount it

```bash
stubr hello.json -p 8080 &
```

#### call it

```bash
curl -i http://localhost:8080/hello/stubr
```

Which should output:

```bash
HTTP/1.1 200 OK
server: stubr(0.4.14)
content-type: text/plain
content-length: 13
date: Sat, 23 Jul 2022 09:25:42 GMT

Hello stubr !%
```