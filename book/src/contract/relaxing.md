# Relaxing your stubs

Whenever you are maintaining a large cohort of interconnected services, you more often than not will opt for Test Data
e.g. a user with firstname `john` and lastname `doe`. And then use this same sample user in all your services tests.
That works actually quite well. That's actually what we did previously, for example

<details open="true">
<summary><b>in this stub</b></summary>

```json
{{#include ../../../actix-producer/stubs/beer-create.json}}
```

</details>

With such a stub, we can imagine we would end up with a test in our consumer like that:

```rust,no_run,noplayground
{{#include ../../../actix-consumer/tests/api/beer.rs:stiff_consumer_test}}
```

Notice the request data in this test. Here, this sample is really dummy but in a real application, `name` is likely to
come from the consumer's own Test Data. But it cannot be any value ; remember, the producer API requires `name` to be
unique. So both producer and consumer Test Data, are now tightly coupled. We are also having specific expectations
regarding the response, for example we are expecting the beer name to be `Heineken` and its price `4` (in this situation
you could also relax your test, but sometimes you can't). And in a big system, your consumer is probably also a
producer, so all its stubs are also going to contain `Heineken` and `4`.

## When things change

Things always change. Sometimes, their format does, for example our beer price can change from ~~`4`~~ to `"4.00"` ;
that's fine, contract testing is made to catch those changes. But imagine if, for whatever functional reason, now prices
in your system can no longer contain 0 cent by needs to be 99 cent: all your Test Data have to change. Now imagine your
system is made of hundreds of microservices, all requiring this beer service. That's a shame, and all because some Test
Data in a single service has changed.

This could have been avoided if we had taken some time to make our stubs relax its request expectation and also return
randomized response data.

## Relaxing fields

Here we will get rid of all the hardcoded data in the response (currently there are not enough helpers to also relax
the request data with stubr, but it will be possible one day).

We will first randomize the id with [anyU32](../stubs/response.md#relaxed-field). On the consumer side, this will
generate a random `u32`. On the producer side in the [verifier tests](producer.md#verify), it simply asserts that the
field is a `u32`.  

Now, for the other fields, those are directly taken from the request, so we'll
use [response templating helpers](../stubs/response.md#response-templating) to forward them from the request.  

We end up with a stub like this:

```json
{{#include ../../../actix-producer/stubs/beer-create-relax.json}}
```
