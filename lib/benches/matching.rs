use criterion::{async_executor::AsyncStdExecutor, black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use surf::{get, post};

use stubr::Stubr;

fn url_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/matching/url");
    let url_path = format!("{}{}", srv.uri(), "/api/exact-uri");
    c.bench_function("matching urlPath", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url_path)))
    });
    let url_pattern = format!("{}{}", srv.uri(), "/api/url-pattern/abcd?one=abcd");
    c.bench_function("matching urlPattern", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url_pattern)))
    });
    let url_path_pattern = format!("{}{}", srv.uri(), "/api/regex-uri/abcd");
    c.bench_function("matching urlPathPattern", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url_path_pattern)))
    });
    let url = format!("{}{}", srv.uri(), "/api/url?age=42");
    c.bench_function("matching url", |b| b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url))));
}

fn query_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/matching/query");
    let query_equal = format!("{}{}", srv.uri(), "/api/query-equal?one=one");
    c.bench_function("matching query by equality", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&query_equal)))
    });
    let query_absence = format!("{}{}", srv.uri(), "/api/query-absence?present=here");
    c.bench_function("matching query by absence", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&query_absence)))
    });
    let query_equal_case_insensitive = format!("{}{}", srv.uri(), "/api/query-equal-case-insensitive?one=ONE");
    c.bench_function("matching query by equality case insensitive", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(get(&query_equal_case_insensitive)))
    });
    let query_regex = format!("{}{}", srv.uri(), "/api/query-regex?one=two");
    c.bench_function("matching query by regex", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&query_regex)))
    });
    let query_contains = format!("{}{}", srv.uri(), "/api/query-contains?one=two");
    c.bench_function("matching query by contains", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&query_contains)))
    });
}

fn body_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/matching/body");
    let eq_to_json_uri = format!("{}{}", srv.uri(), "/api/body-equal-to-json");
    c.bench_function("matching body 'equalToJson'", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&eq_to_json_uri).body(json!({"name":"john.doe"}))))
    });
    let eq_to_json_extra_uri = format!("{}{}", srv.uri(), "/api/body-equal-to-json-extra");
    c.bench_function("matching body 'equalToJson' ignoring extra elements", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&eq_to_json_extra_uri).body(json!({"name":"john.doe", "age": 42}))))
    });
    let eq_to_json_order_uri = format!("{}{}", srv.uri(), "/api/body-equal-to-json-order");
    c.bench_function("matching body 'equalToJson' ignoring array order", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&eq_to_json_order_uri).body(json!({"names":["doe", "john"]}))))
    });
    let matches_json_path_uri = format!("{}{}", srv.uri(), "/api/matches-json-path");
    c.bench_function("matching body single json path", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&matches_json_path_uri).body(json!({"name":"john.doe"}))))
    });
    let matches_many_json_path_uri = format!("{}{}", srv.uri(), "/api/matches-many-json-path");
    c.bench_function("matching body many json path", |b| {
        b.to_async(AsyncStdExecutor).iter(|| {
            black_box(post(&matches_many_json_path_uri).body(json!({"a":1,"b":2,"c":3,"d":4,"e":5,"f":6,"g":7,"h":8,"i":9,"j":10,})))
        })
    });
    let matches_json_path_eq_uri = format!("{}{}", srv.uri(), "/api/matches-json-path-eq");
    c.bench_function("matching body json path eq", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&matches_json_path_eq_uri).body(json!({"consoles": [ { "id": "xbox" } ]}))))
    });
    let matches_json_path_gt_uri = format!("{}{}", srv.uri(), "/api/matches-json-path-gt");
    c.bench_function("matching body json path comparison", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&matches_json_path_gt_uri).body(json!({"consoles": [ { "price": 201 } ]}))))
    });
    let matches_expression_eq_uri = format!("{}{}", srv.uri(), "/api/expression-eq");
    c.bench_function("matching body json path expression equals", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&matches_expression_eq_uri).body(json!({"person": { "name": "john.doe" }}))))
    });
    let matches_expression_contains_uri = format!("{}{}", srv.uri(), "/api/expression-contains");
    c.bench_function("matching body json path expression contains", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| black_box(post(&matches_expression_contains_uri).body(json!({"person": { "name": "john.doe" }}))))
    });
}

fn matching_bench(c: &mut Criterion) {
    url_bench(c);
    query_bench(c);
    body_bench(c);
}

criterion_group!(matching, matching_bench);
criterion_main!(matching);
