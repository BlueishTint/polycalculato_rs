use polycalculato_rs::combat::optimized;
use polycalculato_rs::unit::{Unit, UnitType, Units};

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
            out.push_str(" ➔ ");
            out.push_str(
                &((event.attacker.current_hp - event.damage_to_attacker)
                    .max(0.0)
                    .round() as i32)
                    .to_string(),
            );
            out.push_str("\n");
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
            out.push_str(" ➔ ");
            out.push_str(
                &((event.defender.current_hp - event.damage_to_defender)
                    .max(0.0)
                    .round() as i32)
                    .to_string(),
            );
            out.push_str("\n");
        }

        out
    }
}

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
    let start = std::time::Instant::now();
    println!(
        "{}",
        utils::repr_optim(optimized(attackers, 8, defenders, 1).1)
    );
    println!("Time taken: {:?}", start.elapsed());
}
