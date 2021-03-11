use criterion::{async_executor::AsyncStdExecutor, black_box, Criterion, criterion_group, criterion_main};
use serde_json::json;
use surf::post;

use stubr::Stubr;

fn body_templating_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/templating");
    let uri = format!("{}{}", srv.uri(), "/api/json");
    c.bench_function("template request body", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(post(&uri).body(json!({"name": "jdoe"}))))
    });
}

fn template_bench(c: &mut Criterion) {
    body_templating_bench(c);
}

criterion_group!(templating, template_bench);
criterion_main!(templating);