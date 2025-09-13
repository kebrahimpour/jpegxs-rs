use criterion::{criterion_group, criterion_main, Criterion};
use jpegxs_conformance::benchmarks::MemoryBenchmark;

fn memory_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");

    group.bench_function("4k_encoding", |b| {
        b.iter(|| {
            let mut bench = MemoryBenchmark::new();
            bench.run().expect("Memory benchmark failed");
        });
    });

    group.finish();
}

criterion_group!(benches, memory_benchmark);
criterion_main!(benches);
