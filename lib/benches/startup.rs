use criterion::{black_box, Criterion, criterion_group, criterion_main};
use criterion::async_executor::AsyncStdExecutor;

use stubr::Stubr;

fn startup_bench(c: &mut Criterion) {
    c.bench_function("start 1 file", |b| {
        b.to_async(AsyncStdExecutor).iter(|| {
            Stubr::start(black_box("benches/stubs/startup/one-stub/1.json"))
        })
    });
    c.bench_function("start dir of 1 file", |b| {
        b.to_async(AsyncStdExecutor).iter(|| {
            Stubr::start(black_box("benches/stubs/startup/one-stub"))
        })
    });
    c.bench_function("start dir of 10 files", |b| {
        b.to_async(AsyncStdExecutor).iter(|| {
            Stubr::start(black_box("benches/stubs/startup/ten-stub"))
        })
    });
}

criterion_group!(startup, startup_bench);
criterion_main!(startup);