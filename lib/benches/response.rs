use criterion::{async_executor::AsyncStdExecutor, black_box, criterion_group, criterion_main, Criterion};
use surf::get;

use stubr::Stubr;

fn body_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/response/body");
    let json_uri = format!("{}{}", srv.uri(), "/api/json");
    c.bench_function("json response", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&json_uri)))
    });
    let text_uri = format!("{}{}", srv.uri(), "/api/text");
    c.bench_function("text response", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&text_uri)))
    });
    let file_uri = format!("{}{}", srv.uri(), "/api/file");
    c.bench_function("file response", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&file_uri)))
    });
}

fn header_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/response/header");
    let single_header_uri = format!("{}{}", srv.uri(), "/api/single-header");
    c.bench_function("single header response", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&single_header_uri)))
    });
    let ten_headers_uri = format!("{}{}", srv.uri(), "/api/ten-headers");
    c.bench_function("many headers response", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&ten_headers_uri)))
    });
}

fn response_bench(c: &mut Criterion) {
    body_bench(c);
    header_bench(c);
}

criterion_group!(response, response_bench);
criterion_main!(response);
