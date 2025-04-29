use criterion::{Criterion, black_box, criterion_group, criterion_main};
use polycalculato_rs::{
    combat::optimized,
    unit::{Unit, UnitType, Units},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("optim");
    group.sample_size(10);
    let attackers = Units::from([
        Unit::new(UnitType::Warrior),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider).with_current_hp(6.0),
        Unit::new(UnitType::Archer),
    ]);
    let defenders = Units::from([
        Unit::new(UnitType::Giant),
        Unit::new(UnitType::Archer),
        Unit::new(UnitType::Archer),
        Unit::new(UnitType::Archer),
    ]);

    group.bench_function(
        "wa, ri, ri, ri, ri, ri, ri, ri 6, ar vs gi, ar, ar, ar",
        |b| {
            b.iter(|| {
                optimized(
                    black_box(attackers.clone()),
                    black_box(9),
                    black_box(defenders.clone()),
                    black_box(4),
                )
            });
        },
    );
    group.finish();

    let attackers = Units::from([
        Unit::new(UnitType::Warrior),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider),
        Unit::new(UnitType::Rider).with_current_hp(6.0),
    ]);
    let defenders = Units::from([Unit::new(UnitType::Giant)]);

    c.bench_function("wa, ri, ri, ri, ri, ri, ri, ri 6 vs gi", |b| {
        b.iter(|| {
            optimized(
                black_box(attackers.clone()),
                black_box(8),
                black_box(defenders.clone()),
                black_box(1),
            )
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
