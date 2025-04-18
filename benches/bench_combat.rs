use criterion::{Criterion, black_box, criterion_group, criterion_main};
use polycalculato_rs::{combat, unit};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("wa ar", |b| {
        b.iter(|| {
            combat::single_combat(
                black_box(&unit::Unit::warrior()),
                black_box(&unit::Unit::archer()),
            )
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
