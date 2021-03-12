use criterion::{async_executor::AsyncStdExecutor, black_box, Criterion, criterion_group, criterion_main};
use surf::get;

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
    c.bench_function("matching url", |b| {
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&url)))
    });
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
        b.to_async(AsyncStdExecutor).iter(|| black_box(get(&query_equal_case_insensitive)))
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

fn matching_bench(c: &mut Criterion) {
    url_bench(c);
    query_bench(c);
}

criterion_group!(matching, matching_bench);
criterion_main!(matching);