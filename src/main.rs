use polycalculato_rs::combat::{UnitResult, single_combat};
use polycalculato_rs::unit::{StatusEffects, Unit, UnitType};

fn main() {
    let attacker = Unit::new(UnitType::Warrior);
    let defender = Unit::new(UnitType::Archer);
    let mut result = (
        UnitResult {
            damage: 0.0,
            status_effects: StatusEffects::empty(),
        },
        UnitResult {
            damage: 0.0,
            status_effects: StatusEffects::empty(),
        },
    );
    for _ in 0..100000 {
        result = single_combat(&attacker, &defender);
    }
    println!("{:?}", result);
}
