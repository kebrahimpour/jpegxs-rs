use criterion::{criterion_group, criterion_main, Criterion};
use jpegxs_conformance::benchmarks::SpeedBenchmark;

fn speed_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("speed");
    
    group.bench_function("encode_decode_cycle", |b| {
        b.iter(|| {
            let mut bench = SpeedBenchmark::new();
            bench.run().expect("Speed benchmark failed");
        });
    });
    
    group.finish();
}

criterion_group!(benches, speed_benchmark);
criterion_main!(benches);