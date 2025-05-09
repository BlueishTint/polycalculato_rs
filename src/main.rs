#![allow(unused_imports)]

use polycalculato_rs::combat::{multi_combat_log, multi_combat_score, optimized};
use polycalculato_rs::unit::{StatusEffects, Unit, UnitType, Units};
use utils::repr_optim;

mod utils {
    use polycalculato_rs::{combat::CombatLog, unit::StatusEffects};

    fn status_effects_to_str(status_effects: StatusEffects) -> String {
        let mut out = String::new();
        for status_effect in status_effects.iter_names() {
            if !out.is_empty() {
                out.push_str(", ");
            }
            out.push_str(status_effect.0);
        }

        out
    }

    pub fn repr_optim(log: CombatLog) -> String {
        let mut out = String::new();

        out.push_str("This is the order for the best outcome:\n\n");
        out.push_str("**Attacker (statusEffects): startHP ➔ endHP**\n");

        for event in log.iter() {
            out.push_str(event.attacker.unit_type.into());
            if event.status_effects_to_attacker.is_empty() {
                out.push_str(": ");
            } else {
                out.push_str(&format!(
                    " ({}): ",
                    status_effects_to_str(event.status_effects_to_attacker)
                ));
            }
            out.push_str(&(event.attacker.current_hp.round() as i32).to_string());
            out.push_str(" ➔  ");
            out.push_str(
                &((event.attacker.current_hp - event.damage_to_attacker)
                    .max(0.0)
                    .round() as i32)
                    .to_string(),
            );
            out.push('\n');
        }

        out.push_str("\n**Defender (statusEffects): startHP ➔ endHP**\n");
        for event in log.iter() {
            out.push_str(event.defender.unit_type.into());
            if event.status_effects_to_defender.is_empty() {
                out.push_str(": ");
            } else {
                out.push_str(&format!(
                    " ({}): ",
                    status_effects_to_str(event.status_effects_to_attacker)
                ));
            }
            out.push_str(&(event.defender.current_hp.round() as i32).to_string());
            out.push_str(" ➔  ");
            out.push_str(
                &((event.defender.current_hp - event.damage_to_defender)
                    .max(0.0)
                    .round() as i32)
                    .to_string(),
            );
            out.push('\n');
        }

        out
    }
}

fn main() {
    let attackers = Units::from([
        Unit::new(UnitType::Rider).with_current_hp(6.0),
        Unit::new(UnitType::Warrior),
        Unit::new(UnitType::Warrior),
        Unit::new(UnitType::Warrior),
        Unit::new(UnitType::Knight),
        Unit::new(UnitType::Catapult),
        Unit::new(UnitType::Archer).with_current_hp(3.0),
        Unit::new(UnitType::Catapult)
            .with_current_hp(5.0)
            .with_status_effects(StatusEffects::VETERAN),
        Unit::new(UnitType::Archer).with_current_hp(3.0),
        Unit::new(UnitType::Catapult)
            .with_current_hp(5.0)
            .with_status_effects(StatusEffects::VETERAN),
    ]);
    let defenders = Units::from([
        Unit::new(UnitType::Giant),
        Unit::new(UnitType::Giant),
        Unit::new(UnitType::Giant),
    ]);
    // dbg!(multi_combat_log(&attackers, defenders));
    // println!("{}", repr_optim(multi_combat_log(&attackers, defenders)));

    let start_time = std::time::Instant::now();
    println!("{}", repr_optim(optimized(attackers, defenders).1));
    println!("{:?}", std::time::Instant::now() - start_time);
}
