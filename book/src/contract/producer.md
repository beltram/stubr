# As a producer

We'll use actix in those examples because it is the only web framework currently supported for verifying the producer,
others will come ! And of course it does not impact the consumer where you can use any technology you want since [stubr](https://github.com/beltram/stubr)
can be used [standalone](../getting-started/standalone.md) (you can even use a language other than Rust !).

## endpoint

We'll begin in a very common situation where your producer exposes a http endpoint. We will make the endpoint as simple
and stupid as possible with a flat `Beer` resource and an in-memory database. We'll just expose an endpoint for creating
and fetching a resource which should cover most of the use cases.

```rust,ignore,noplayground,edition2021
{{#include ../../../actix-producer/src/api/beer.rs}}
```

## tests

Then as we do things seriously, we will write some tests. For the `create` endpoint we'll have a nominal case where the 
beer we create succeeds and another negative one where it fails because we have a uniqueness constraint on the Beer's 
`name` and `price` fields. For the `find_by_id` one, we'll have a nominal case and a `404 Not Found` one.

```rust,ignore,noplayground,edition2021
{{#include ../../../actix-producer/tests/api/beer.rs}}
```

## recording

Although it is optional, we'll use [recording](../recording/actix.md) here to easily create one stub for each test. 
Recording is triggered by the `.wrap(stubr::ActixRecord::default())` line. Those recorded stubs will be in 
`target/stubs/localhost`. 

## import stubs

In order for a producer to expose stubs, they have to live in a `stubs` folder in the root of your project. So 
copy/paste the 4 recorded stubs into this folder and arrange them a bit (remove recording noise) to match the following.  

<details>
<summary><b>create</b></summary>

```json
{{#include ../../../actix-producer/stubs/beer-create.json}}
```

</details>

<details>
<summary><b>create with name conflict</b></summary>

```json
{{#include ../../../actix-producer/stubs/beer-create-conflict-name.json}}
```

</details>

<details>
<summary><b>find by id</b></summary>

```json
{{#include ../../../actix-producer/stubs/beer-find-by-id.json}}
```

</details>

<details>
<summary><b>find by id not found</b></summary>

```json
{{#include ../../../actix-producer/stubs/beer-find-by-id-not-found.json}}
```

</details>

## verify

And finally, we have to verify that the stubs exposed by our producer match the actual implementation. To do so, [stubr](https://github.com/beltram/stubr)
exports the `StubrVerify` trait with the `.verify()` method you have to invoke. There is no automatic verification of
stubs possible, it has to be explicit in a test. It is advised to declare it in a file with just the verification test.    

Such a test will start by declaring your actix app with all the endpoints. In order to verify it, [stubr](https://github.com/beltram/stubr) will create a
test for every stub in `./stubs` by converting, for each, the `request` part in an actual actix integration test.  

But you might need some state ! For example, think of the `find by id` endpoint. It cannot be verified if your database
is empty. Likewise, stubs are verified in no particular order (since anyway your endpoint are most likely stateless, 
right ?). Executing some tests (for example a "delete" endpoint) might affect others. So before each test, we have to
reset our application state. You can do that with the `stubr::ActixVerifyLifecycle` middleware, for example here to
wipe our database then populate it with our sample data (it is recommended to reuse the same as in your tests).

Finally, call `.verify()` (a bit different in our example) to launch the verification test. If it passes, you have the
guarantee your stubs accurately represent your application API. 

```rust,ignore,noplayground,edition2021
{{#include ../../../actix-producer/tests/api/verify_book.rs}}
```

Now let's use those stubs in a consumer.
