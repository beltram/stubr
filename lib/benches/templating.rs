use criterion::{async_executor::AsyncStdExecutor, black_box, Criterion, criterion_group, criterion_main};
use serde_json::json;
use surf::post;

use stubr::Stubr;

fn body_templating_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/templating");
    let req_body_uri = format!("{}{}", srv.uri(), "/api/request-body");
    c.bench_function("template request body", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(post(&req_body_uri).body(json!({"name": "jdoe"}))))
    });
    let jsonpath_uri = format!("{}{}", srv.uri(), "/api/jsonpath");
    c.bench_function("template request body by jsonpath", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(post(&jsonpath_uri).body(json!({"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}))))
    });
}

fn template_bench(c: &mut Criterion) {
    body_templating_bench(c);
}

criterion_group!(templating, template_bench);
criterion_main!(templating);