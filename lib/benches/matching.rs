use criterion::{async_executor::AsyncStdExecutor, black_box, Criterion, criterion_group, criterion_main};
use surf::get;

use stubr::Stubr;

fn url_bench(c: &mut Criterion) {
    let srv = Stubr::start_blocking("benches/stubs/matching/url");
    let url_path = format!("{}{}", srv.uri(), "/api/exact-uri");
    c.bench_function("matching urlPath", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url_path)))
    });
    let url_path_pattern = format!("{}{}", srv.uri(), "/api/regex-uri/abcd");
    c.bench_function("matching urlPathPattern", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url_path_pattern)))
    });
    let url = format!("{}{}", srv.uri(), "/api/url?age=42");
    c.bench_function("matching url", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url)))
    });
}

fn matching_bench(c: &mut Criterion) {
    url_bench(c);
}

criterion_group!(matching, matching_bench);
criterion_main!(matching);