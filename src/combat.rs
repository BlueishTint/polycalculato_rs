use crate::unit::{StatusEffects, Traits, UnitKind};

fn calculate_damage(
    attack: f32,
    defense: f32,
    attacker_health_ratio: f32,
    defender_health_ratio: f32,
    defense_bonus: f32,
    halved: bool,
) -> [f32; 2] {
    let attack_force = attack * attacker_health_ratio;
    let defense_force = defense * defender_health_ratio * defense_bonus;
    let total_damage = attack_force + defense_force;

    if total_damage == 0.0 {
        return [0.0, 0.0];
    }

    let to_defender = (attack_force / total_damage * attack * 4.5).round();
    let to_attacker = (defense_force / total_damage * defense * 4.5).round();

    if halved {
        [to_attacker / 2.0, to_defender / 2.0]
    } else {
        [to_attacker, to_defender]
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

    let to_defender = (attack_force / total_damage * attack * 4.5).round();

    to_defender
}

fn calculate_status_effects(
    attacker_traits: Traits,
    defender_traits: Traits,
) -> [StatusEffects; 2] {
    let mut to_attacker = StatusEffects::empty();
    let mut to_defender = StatusEffects::empty();

    if attacker_traits.contains(Traits::POISON) {
        to_defender.insert(StatusEffects::POISONED);
    }
    if attacker_traits.contains(Traits::CONVERT) {
        to_defender.insert(StatusEffects::CONVERTED);
    }
    if attacker_traits.contains(Traits::FREEZE) {
        to_defender.insert(StatusEffects::FROZEN);
    }
    if defender_traits.contains(Traits::POISON) {
        to_attacker.insert(StatusEffects::POISONED);
    }

    [to_attacker, to_defender]
}

#[derive(Debug)]
pub struct UnitResult {
    pub damage: f32,
    pub status_effects: StatusEffects,
}

pub fn single_combat(attacker: &UnitKind, defender: &UnitKind) -> (UnitResult, UnitResult) {
    let attacker_traits = attacker.traits();
    let defender_traits = defender.traits();

    let mut tentacle_damage = 0f32;
    let mut takes_retaliation = false;

    if defender_traits.contains(Traits::TENTACLES) {
        if attacker_traits.contains(Traits::TENTACLES) {
            takes_retaliation = true;
        } else if attacker.range() <= defender.range() {
            tentacle_damage = calculate_attacker_damage(
                defender.attack(),
                attacker.defense(),
                defender.health_ratio(),
                attacker.health_ratio(),
                attacker.defense_bonus(),
            );
        }
    }

    let [damage_to_attacker, damage_to_defender] = calculate_damage(
        attacker.attack(),
        defender.defense(),
        (attacker.current_hp() - tentacle_damage) / attacker.max_hp(),
        defender.health_ratio(),
        defender.defense_bonus(),
        attacker
            .status_effects()
            .contains(StatusEffects::SPLASHING | StatusEffects::EXPLODING),
    );

    takes_retaliation = takes_retaliation
        || attacker
            .status_effects()
            .contains(StatusEffects::TAKES_RETALIATION)
        || !(attacker.range() > defender.range()
            || (defender.current_hp() - damage_to_defender) <= 0.0
            || defender_traits.contains(Traits::STIFF)
            || attacker_traits.contains(Traits::SURPRISE)
            || attacker_traits.contains(Traits::CONVERT)
            || attacker_traits.contains(Traits::FREEZE)
            || defender.status_effects().contains(StatusEffects::FROZEN));

    let [effects_to_attacker, effects_to_defender] =
        calculate_status_effects(attacker_traits, defender_traits);

    let exploding = attacker.status_effects().contains(StatusEffects::EXPLODING);

    // Premature optimization to avoid branches
    //
    // let total_damage_to_attacker = (damage_to_attacker * f32::from(takes_retaliation)
    //     + tentacle_damage)
    //     * f32::from(!exploding)
    //     + f32::from(exploding) * attacker.max_hp();

    let total_damage_to_attacker = if exploding {
        attacker.max_hp()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wa_wa() {
        let attacker = UnitKind::warrior();
        let defender = UnitKind::warrior();

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 5.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_wa_wa_d() {
        let attacker = UnitKind::warrior();
        let defender = UnitKind::Regular(crate::unit::Unit::new(
            crate::unit::UnitType::Warrior,
            None,
            Some(StatusEffects::FORTIFIED),
        ));

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 4.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_wa_je() {
        let attacker = UnitKind::warrior();
        let defender = UnitKind::jelly();

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 3.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }

    #[test]
    fn test_je_wa() {
        let attacker = UnitKind::jelly();
        let defender = UnitKind::warrior();

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 5.0);
        assert_eq!(defender_result.damage, 5.0);
        assert_eq!(attacker_result.status_effects, StatusEffects::empty());
        assert_eq!(defender_result.status_effects, StatusEffects::empty());
    }
}
