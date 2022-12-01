// ANCHOR: contract_consumer_test
use asserhttp::*;
use serde_json::json;

#[test]
#[stubr::apps("actix-producer")]
fn find_by_id_should_find_one() {
    let uri: String = actix_producer.uri();
    let beer_id = 0;
    reqwest::blocking::get(format!("{uri}/beers/{beer_id}"))
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({
            "id": 0,
            "name": "Leffe",
            "price": 5
        }));
}

#[test]
#[stubr::apps("actix-producer")]
fn find_by_id_should_not_find_any() {
    let uri: String = actix_producer.uri();
    reqwest::blocking::get(format!("{uri}/beers/404"))
        .expect_status_not_found()
        .expect_content_type_json()
        .expect_body_json_eq(json!({
            "message": "Beer not found"
        }));
}

// ANCHOR: stiff_consumer_test
#[test]
#[stubr::apps("actix-producer")]
fn create_should_create_one() {
    let uri: String = actix_producer.uri();
    reqwest::blocking::Client::new()
        .post(format!("{uri}/beers"))
        .json(&json!({
            "name": "Heineken",
            "price": 4
        }))
        .send()
        .expect_status_created()
        .expect_content_type_json()
        .expect_body_json(|beer: serde_json::Value| {
            assert!(beer.get("name").unwrap().is_string());
            assert!(beer.get("price").unwrap().is_u64());
        });
}
// ANCHOR_END: stiff_consumer_test

#[test]
#[stubr::apps("actix-producer")]
fn create_should_should_conflict_on_name() {
    let uri: String = actix_producer.uri();
    reqwest::blocking::Client::new()
        .post(format!("{uri}/beers"))
        .json(&json!({
            "name": "Leffe",
            "price": 5
        }))
        .send()
        .expect_status_conflict()
        .expect_content_type_json()
        .expect_body_json_eq(json!({
            "message": "Beer already exists"
        }));
}
// ANCHOR_END: contract_consumer_test

// ANCHOR: sample_apps_binding
#[test]
#[stubr::apps("actix-producer")]
fn sample_binding() {
    let actix_producer: stubr::Stubr = actix_producer;
    let _uri: String = actix_producer.uri();
}
// ANCHOR_END: sample_apps_binding
