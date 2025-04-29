use polycalculato_rs::combat::optimized;
use polycalculato_rs::unit::{Unit, UnitType, Units};

fn main() {
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
    dbg!(optimized(attackers, 8, defenders, 1));
}
