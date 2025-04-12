use criterion::{Criterion, black_box, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
