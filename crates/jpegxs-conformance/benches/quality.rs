use criterion::{criterion_group, criterion_main, Criterion};
use jpegxs_conformance::benchmarks::QualityBenchmark;

fn quality_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("quality");

    group.bench_function("compression_quality", |b| {
        b.iter(|| {
            let mut bench = QualityBenchmark::new();
            bench.run().expect("Quality benchmark failed");
        });
    });

    group.finish();
}

criterion_group!(benches, quality_benchmark);
criterion_main!(benches);
