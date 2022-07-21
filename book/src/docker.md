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
server: stubr(0.5.0-rc.2)

Hello stubr
```