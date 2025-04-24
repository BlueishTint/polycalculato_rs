use crate::unit::{StatusEffects, Unit};

const ELIPSON: f32 = 1e-6;

fn calculate_damage(
    attack: f32,
    defense: f32,
    attacker_health_ratio: f32,
    defender_health_ratio: f32,
    defense_bonus: f32,
    halved: bool,
) -> (f32, f32) {
    let attack_force = attack * attacker_health_ratio;
    let defense_force = defense * defender_health_ratio * defense_bonus;
    let total_damage = attack_force + defense_force;

    if total_damage == 0.0 {
        return (0.0, 0.0);
    }

    let to_defender = (attack_force / total_damage * attack * 4.5 + ELIPSON).round();
    let to_attacker = (defense_force / total_damage * defense * 4.5 + ELIPSON).round();

    if halved {
        (to_attacker / 2.0, to_defender / 2.0)
    } else {
        (to_attacker, to_defender)
    }
}

fn calculate_attacker_damage(
    attack: f32,
    defense: f32,
    attacker_health_ratio: f32,
    defender_health_ratio: f32,
    defense_bonus: f32,
) -> f32 {
    let attack_force = attack * attacker_health_ratio;
    let defense_force = defense * defender_health_ratio * defense_bonus;
    let total_damage = attack_force + defense_force;

    if total_damage == 0.0 {
        return 0.0;
    }

    let to_defender = (attack_force / total_damage * attack * 4.5 + ELIPSON).round();

    to_defender
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnitResult {
    pub damage: f32,
    pub status_effects: StatusEffects,
}

#[inline]
fn is_jelly(unit: &Unit) -> bool {
    !unit.retaliates && unit.max_hp == 20.0
}

pub fn single_combat(attacker: &Unit, defender: &Unit) -> (UnitResult, UnitResult) {
    let mut tentacle_damage = 0.0;
    let mut takes_retaliation = false;

    let defender_in_range = defender.range >= attacker.range;

    if is_jelly(defender) {
        if is_jelly(attacker) {
            takes_retaliation = true;
        } else if defender_in_range {
            tentacle_damage = calculate_attacker_damage(
                defender.attack,
                attacker.defense,
                defender.current_hp / defender.max_hp,
                attacker.current_hp / attacker.max_hp,
                attacker.defense_bonus,
            );
        }
    }

    let (damage_to_attacker, damage_to_defender) = calculate_damage(
        attacker.attack,
        defender.defense,
        (attacker.current_hp - tentacle_damage) / attacker.max_hp,
        defender.current_hp / defender.max_hp,
        defender.defense_bonus,
        attacker
            .status_effects
            .contains(StatusEffects::SPLASHING | StatusEffects::EXPLODING),
    );

    let effects_to_defender = attacker.trait_effects;

    takes_retaliation = takes_retaliation
        || attacker
            .status_effects
            .contains(StatusEffects::TAKES_RETALIATION)
        || !(!defender_in_range
            || (defender.current_hp - damage_to_defender) <= 0.0
            || !defender.retaliates
            || attacker.surprise
            || effects_to_defender.contains(StatusEffects::FROZEN)
            || effects_to_defender.contains(StatusEffects::CONVERTED)
            || defender.status_effects.contains(StatusEffects::FROZEN));

    let effects_to_attacker = defender.trait_effects;

    let exploding = attacker.status_effects.contains(StatusEffects::EXPLODING);

    // Premature optimization to avoid branches
    //
    // let total_damage_to_attacker = (damage_to_attacker * f32::from(takes_retaliation)
    //     + tentacle_damage)
    //     * f32::from(!exploding)
    //     + f32::from(exploding) * attacker.max_hp;

    let total_damage_to_attacker = if exploding {
        attacker.max_hp
    } else if takes_retaliation {
        damage_to_attacker + tentacle_damage
    } else {
        tentacle_damage
    };

    // Same optimization can be applied here

    let total_effects_to_attacker = if takes_retaliation {
        effects_to_attacker
    } else {
        StatusEffects::empty()
    };

    (
        UnitResult {
            damage: total_damage_to_attacker,
            status_effects: total_effects_to_attacker,
        },
        UnitResult {
            damage: damage_to_defender,
            status_effects: effects_to_defender,
        },
    )
}

pub fn multi_combat_score<I1, I2>(attackers: I1, defenders: I2) -> f32
where
    I1: IntoIterator<Item = Unit>,
    I2: IntoIterator<Item = Unit>,
{
    let mut score = 0.0;
    let mut defenders = defenders.into_iter();
    let mut defender = defenders.next();
    if defender.is_none() {
        return score;
    }

    for attacker in attackers {
        let defender_mut = defender.as_mut().unwrap();

        let (to_attacker, to_defender) = single_combat(&attacker, &defender_mut);

        score -= if to_attacker.damage > attacker.current_hp {
            attacker.current_hp
        } else {
            to_attacker.damage
        };

        if to_defender.damage >= defender_mut.current_hp {
            score += defender_mut.current_hp;
            defender = defenders.next();
            if defender.is_none() {
                break;
            }
            continue;
        } else {
            score += to_defender.damage;
        };

        defender_mut.current_hp -= to_defender.damage;
        defender_mut
            .status_effects
            .insert(to_defender.status_effects);
    }

    score
}

pub struct CombatEvent {
    pub attacker: Unit,
    pub defender: Unit,
    pub damage_to_attacker: i32,
    pub damage_to_defender: i32,
    pub status_effects_to_attacker: StatusEffects,
    pub status_effects_to_defender: StatusEffects,
}

#[cfg(test)]
mod tests {
    use crate::unit::UnitType;

    use super::*;

    #[test]
    fn test_wa_wa() {
        let attacker = Unit::new(UnitType::Warrior);
        let defender = Unit::new(UnitType::Warrior);

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 5.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_wa_wa_d() {
        let attacker = Unit::new(UnitType::Warrior);
        let defender = Unit::new(UnitType::Warrior).with_status_effects(StatusEffects::FORTIFIED);

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 4.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_wa_je() {
        let attacker = Unit::new(UnitType::Warrior);
        let defender = Unit::new(UnitType::Jelly);

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 3.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_je_wa() {
        let attacker = Unit::new(UnitType::Jelly);
        let defender = Unit::new(UnitType::Warrior);

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 5.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_je_4_wa() {
        let attacker = Unit::new(UnitType::Jelly).with_current_hp(4.0);
        let defender = Unit::new(UnitType::Warrior);

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);
        assert_eq!(attacker_result.damage, 8.0);
        assert_eq!(defender_result.damage, 2.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_wa_wa_vs_wa_d() {
        let attackers = [Unit::new(UnitType::Warrior), Unit::new(UnitType::Warrior)];
        let defenders =
            [Unit::new(UnitType::Warrior).with_status_effects(StatusEffects::FORTIFIED)];

        let score = multi_combat_score(attackers, defenders);

        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_wa_wa_wa_vs_wa_d() {
        let attackers = [
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
        ];
        let defenders =
            [Unit::new(UnitType::Warrior).with_status_effects(StatusEffects::FORTIFIED)];

        let score = multi_combat_score(attackers, defenders);

        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_wa_wa_wa_wa_wa_vs_je() {
        let attackers = [
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
        ];
        let defenders = [Unit::new(UnitType::Jelly)];

        let score = multi_combat_score(attackers, defenders);
        assert_eq!(score, 2.0);
    }
}
