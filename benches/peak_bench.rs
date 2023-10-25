use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peak_result::Peak;

fn peak_err_bench(res: Result<&str, &str>) {
    let _ = res.peak_err(|e| something(e));
}

fn map_err_bench(res: Result<&str, &str>) {
    let _ = res.map_err(|e| something(e));
}

fn something(_e: &str) {}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison");

    group.bench_function("Peak err", |b| {
        b.iter(|| peak_err_bench(black_box(Err("Error"))))
    });

    group.bench_function("Map Err", |b| {
        b.iter(|| map_err_bench(black_box(Err("Error"))))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
