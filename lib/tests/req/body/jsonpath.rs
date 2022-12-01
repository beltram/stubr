use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("req/body/json-path/single.json")]
async fn should_map_req_body_when_one_field_matches() {
    post(stubr.uri()).body(json!({"name": "bob"})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/single.json")]
async fn should_fail_when_none_matches() {
    post(stubr.uri())
        .body(json!({"notName": "bob"}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/many.json")]
async fn should_map_req_body_when_many_fields_match() {
    post(stubr.uri())
        .body(json!({"name": "bob", "age": 42}))
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/many.json")]
async fn should_fail_when_one_of_many_does_not_match() {
    post(stubr.uri())
        .body(json!({"notName": "bob", "age": 42}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"name": "bob", "notAge": 42}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({"name": "bob"})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"age": 42})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/plus-eq.json")]
async fn can_be_combined_with_eq() {
    post(stubr.uri())
        .body(json!({"person": { "name": "bob" }}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"person": { "name": "bob" }, "person": { "name": "bob" }}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"person": { "name": "bob" }, "notPerson": { "name": "bob" }}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"person": { "name": "bob" }, "person": { "name": "alice" }}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"person": { "name": "alice" }}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"person": { "notName": "bob" }}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"notPerson": { "name": "bob" }}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({"person": { }})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/plus-contains.json")]
async fn can_be_combined_with_contains() {
    post(stubr.uri()).body(json!({"name": "bob"})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"name": "alice"})).await.expect_status_not_found();
    post(stubr.uri())
        .body(json!({"notName": "bob"}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({"name": ""})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/eq.json")]
async fn should_map_req_body_by_json_path_eq() {
    post(stubr.uri())
        .body(json!({"consoles": [ { "id": "xbox" } ]}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"consoles": [ { "id": "xbox" }, { "id": "playstation" } ]}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"consoles": [ { "id": "playstation" }, { "id": "xbox" } ]}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"consoles": [ { "id": "playstation" }, { "id": "xbox" }, { "id": "switch" } ]}))
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/eq.json")]
async fn should_fail_when_json_path_not_equals() {
    post(stubr.uri())
        .body(json!({"consoles": [ { "id": "playstation" } ]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"consoles": [ {} ]}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({"consoles": []})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/json-path/gt.json")]
async fn should_map_req_body_by_json_path_greater_than() {
    post(stubr.uri())
        .body(json!({"consoles": [ { "price": 201 } ]}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"consoles": [ { "price": 200 } ]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"consoles": [ { "price": 199 } ]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"consoles": [ { "price": 201 }, { "price": 199 } ]}))
        .await
        .expect_status_ok();
    post(stubr.uri())
        .body(json!({"consoles": [ { "price": 199 }, { "price": 199 } ]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"consoles": [ { } ]}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({"consoles": [ ]})).await.expect_status_not_found();
}
