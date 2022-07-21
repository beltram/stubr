# Stubr

Stubr is a stubbing and contract testing tool. It supports the same API for writing stubs
as [Wiremock](https://github.com/tomakehurst/wiremock).
You can also see it as an adaptation of [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) with the ability to
write stubs as json files instead of code.

You might ask **why would I want to do such a thing ?** After all, code is a great way to write stubs. That's true ! But
it also comes with some limitations: **it is hard to share.** And it especially starts tickling your attention when you
try to do [contract testing](https://martinfowler.com/bliki/ContractTest.html). You have to share a `contract` between a
producer and a consumer. Sometimes, both are written in different languages, or with different frameworks ; and even
when that's not the case, they might both be sharing a different version of that wicked dependency which clash together.
Having your contract written in json has the benefits of being **portable**, **lightweight** and **polyglot** (if you
stick to a
standard API such as Wiremock's one). So you could for example test a producer service written in Java and vice versa !

Stubr aims at bridging multiple languages and framework and enable developers to test their integration with remote
application without having to mock their own code. It also enables them to shorten their feedback loop: no need for a
complex CI to make sure 2 application share the same API definition, everything can be done offline.

Then, beyond [contract testing](contract/index.md), it tries to cover others areas:

* [stubbing](getting-started/unit-test.md) in your Rust project for simulating remote services your app depends on
* [recording](recording/index.md) for capturing http traffic and turning it into a stub file
* [standalone](getting-started/standalone.md) for running stubr stub server from anywhere and benefiting from Rust performances. Available as
  a [cli](cli.md) or a [Docker image](docker.md)