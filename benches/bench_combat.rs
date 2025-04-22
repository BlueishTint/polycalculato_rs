use criterion::{Criterion, black_box, criterion_group, criterion_main};
use polycalculato_rs::{combat, unit};

pub fn criterion_benchmark(c: &mut Criterion) {
    let attacker = unit::Unit::new(unit::UnitType::Warrior);
    let defender = unit::Unit::new(unit::UnitType::Archer);

    c.bench_function("wa ar", |b| {
        b.iter(|| combat::single_combat(black_box(&attacker), black_box(&defender)));
    });

    let attacker = unit::Unit::new(unit::UnitType::Warrior);
    let defender = unit::Unit::new(unit::UnitType::DefaultWarriorRaft);

    c.bench_function("wa rf", |b| {
        b.iter(|| combat::single_combat(black_box(&attacker), black_box(&defender)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
