use criterion::{Criterion, black_box, criterion_group, criterion_main};
use polycalculato_rs::{combat, unit};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("wa ar", |b| {
        b.iter(|| {
            combat::single_combat(
                black_box(&unit::UnitKind::warrior()),
                black_box(&unit::UnitKind::archer()),
            )
        });
    });
    c.bench_function("wa rf", |b| {
        b.iter(|| {
            combat::single_combat(
                black_box(&unit::UnitKind::warrior()),
                black_box(&unit::UnitKind::raft(None)),
            )
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
