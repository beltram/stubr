# Recording with the cli

In order to record http traffic, [stubr](https://github.com/beltram/stubr) can act as a proxy to dump this traffic into json stubs on your local
filesystem. Recording can be started with the `stubr record` command. Stubs will be grouped by hosts. You can then play
them back using [stubr](https://github.com/beltram/stubr).

| arg        |                                  about                                   |                    examples                    |
|------------|:------------------------------------------------------------------------:|:----------------------------------------------:|
| `--port`   |                      Proxy port. Defaults to 3030.                       |     `stubr --port 3031` or `stubr -p 3031`     |
| `--output` | File path where recorded stubs are stored. Default to current directory. | `stubr --port record-1` or `stubr -o record-1` |

### example

First, start [stubr](https://github.com/beltram/stubr) recorder on port `3030`. It will act as a proxy.

```bash
stubr record -p 3030
```

We are going to consume a publicly available endpoint returning a list of sample users. We'll use [curl](https://curl.se/)
to make this http call, and we will configure it to use our recorder as a proxy.

```bash
curl jsonplaceholder.typicode.com/users --proxy http://localhost:3030
```
You should have a stub under `jsonplaceholder.typicode.com/users-*.json` following the pattern `{domain}/{path}-{md5-hash}.json`.

NB: *That way of recording is less intrusive than [if you had to do it with wiremock,](https://wiremock.org/docs/record-playback/)
and you can configure. Most of the tools e.g. [curl](https://curl.se/), [JMeter](https://jmeter.apache.org/), [k6](https://k6.io/) or
simply your web browser support configuring a http proxy (and more often than not, just by setting an environment variable,
leaving your tests/scripts untouched).*