use crate::{
    unit::{StatusEffects, Unit, UnitType, Units},
    utils::Perms,
};

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

    let to_defender = (attack_force / total_damage * attack.mul_add(4.5, ELIPSON)).round();
    let to_attacker = (defense_force / total_damage * defense.mul_add(4.5, ELIPSON)).round();

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

    (attack_force / total_damage * attack.mul_add(4.5, ELIPSON)).round()
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnitResult {
    pub damage: f32,
    pub status_effects: StatusEffects,
}

pub fn single_combat(attacker: &Unit, defender: &Unit) -> (UnitResult, UnitResult) {
    let mut tentacle_damage = 0.0;
    let mut takes_retaliation = false;

    let defender_in_range = defender.range >= attacker.range;

    if defender.unit_type == UnitType::Jelly {
        if attacker.unit_type == UnitType::Jelly {
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

pub fn multi_combat_score(attackers: &Units, mut defenders: Units) -> f32 {
    let n_defenders = defenders.len();
    let mut score = 0.0;
    let mut d_idx = 0;

    for a_idx in 0..attackers.len() {
        let attacker = &attackers[a_idx];
        if d_idx >= n_defenders {
            break;
        }

        let defender = &mut defenders[d_idx];

        let (to_attacker, to_defender) = single_combat(attacker, defender);

        score -= to_attacker.damage.min(attacker.current_hp);
        score += to_defender.damage.min(defender.current_hp);

        if to_defender.damage >= defender.current_hp {
            d_idx += 1;
            score += 100.0;
        } else {
            defender.current_hp -= to_defender.damage;
            defender.apply_status_effects(to_defender.status_effects);
        }
    }

    score
}

#[derive(Debug)]
pub struct CombatEvent {
    pub attacker: Unit,
    pub defender: Unit,
    pub damage_to_attacker: f32,
    pub damage_to_defender: f32,
    pub status_effects_to_attacker: StatusEffects,
    pub status_effects_to_defender: StatusEffects,
}

#[derive(Debug)]
pub struct CombatLog(Vec<CombatEvent>);

impl CombatLog {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Default for CombatLog {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for CombatLog {
    type Target = Vec<CombatEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CombatLog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn multi_combat_log(attackers: &Units, mut defenders: Units) -> CombatLog {
    let mut log = CombatLog::new();
    let mut d_idx = 0;

    let n_defenders = defenders.len();

    for a_idx in 0..attackers.len() {
        let attacker = &attackers[a_idx];
        if d_idx >= n_defenders {
            break;
        }

        let defender = &mut defenders[d_idx];

        let (to_attacker, to_defender) = single_combat(attacker, defender);

        log.push(CombatEvent {
            attacker: attacker.clone(),
            defender: defender.clone(),
            damage_to_attacker: to_attacker.damage,
            damage_to_defender: to_defender.damage,
            status_effects_to_attacker: to_attacker.status_effects,
            status_effects_to_defender: to_defender.status_effects,
        });

        if to_defender.damage >= defender.current_hp {
            d_idx += 1;
        } else {
            defender.current_hp -= to_defender.damage;
            defender.apply_status_effects(to_defender.status_effects);
        }
    }

    log
}

pub fn optimized(mut attackers: Units, mut defenders: Units) -> (f32, CombatLog) {
    let attacker_pairs = Perms::new(attackers.len());
    let n_defenders = defenders.len();

    let mut top_score = f32::MIN;
    let mut best_attacker_order: Units = Units::new();
    let mut best_defender_order: Units = Units::new();
    for (a_first, a_second) in attacker_pairs {
        attackers.swap(a_first, a_second);

        let defenders_perms = Perms::new(n_defenders);
        for (d_first, d_second) in defenders_perms {
            defenders.swap(d_first, d_second);

            let score = multi_combat_score(&attackers, defenders.clone());
            if score > top_score {
                top_score = score;
                best_attacker_order = attackers.clone();
                best_defender_order = defenders.clone();
            }
        }
    }

    (
        top_score,
        multi_combat_log(&best_attacker_order, best_defender_order),
    )
}

pub fn bulk(attacker: &Unit, mut defender: Unit) -> u32 {
    let mut n_attacks = 0;

    while defender.current_hp > 0.0 {
        n_attacks += 1;

        let damage_to_defender = calculate_attacker_damage(
            attacker.attack,
            defender.defense,
            attacker.current_hp / attacker.max_hp,
            defender.current_hp / defender.max_hp,
            defender.defense_bonus,
        );

        defender.current_hp -= damage_to_defender;
        defender.apply_status_effects(attacker.trait_effects);
    }

    n_attacks
}

pub fn eliminate_defender(attacker: &Unit, mut defender: Unit) -> f32 {
    let mut i: u16 = 0;

    loop {
        defender.current_hp = defender.max_hp - i as f32;

        let damage_to_defender = calculate_attacker_damage(
            attacker.attack,
            defender.defense,
            attacker.current_hp / attacker.max_hp,
            defender.current_hp / defender.max_hp,
            defender.defense_bonus,
        );

        if defender.current_hp <= damage_to_defender {
            break;
        }
        i += 1;
    }

    defender.max_hp - (i as f32)
}

pub fn eliminate_attacker(mut attacker: Unit, defender: &Unit) -> f32 {
    let mut i: u16 = 0;

    loop {
        attacker.current_hp = i as f32;

        let damage_to_defender = calculate_attacker_damage(
            attacker.attack,
            defender.defense,
            attacker.current_hp / attacker.max_hp,
            defender.current_hp / defender.max_hp,
            defender.defense_bonus,
        );

        if defender.current_hp <= damage_to_defender {
            break;
        }

        i += 1;
    }

    i as f32
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
    fn test_wa_wa_d_6() {
        let attacker = Unit::new(UnitType::Warrior);
        let defender = Unit::new(UnitType::Warrior)
            .with_status_effects(StatusEffects::FORTIFIED)
            .with_current_hp(6.0);

        let (attacker_result, defender_result) = single_combat(&attacker, &defender);

        assert_eq!(attacker_result.damage, 4.0);
        assert_eq!(defender_result.damage, 5.0);
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
        let attackers = Units::from([Unit::new(UnitType::Warrior), Unit::new(UnitType::Warrior)]);
        let defenders = Units::from([
            Unit::new(UnitType::Warrior).with_status_effects(StatusEffects::FORTIFIED)
        ]);
        let score = multi_combat_score(&attackers, defenders);

        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_wa_wa_wa_vs_wa_d() {
        let attackers = Units::from([
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
        ]);
        let defenders = Units::from([
            Unit::new(UnitType::Warrior).with_status_effects(StatusEffects::FORTIFIED)
        ]);
        let score = multi_combat_score(&attackers, defenders);

        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_wa_wa_wa_wa_wa_vs_je() {
        let attackers = Units::from([
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Warrior),
        ]);
        let defenders = Units::from([Unit::new(UnitType::Jelly)]);

        let score = multi_combat_score(&attackers, defenders);
        assert_eq!(score, 2.0);
    }

    #[test]
    fn test_ri_6_wa_ar_3_kn_ca_5_v_ca_vs_gi() {
        let attackers = Units::from([
            Unit::new(UnitType::Rider).with_current_hp(6.0),
            Unit::new(UnitType::Warrior),
            Unit::new(UnitType::Archer).with_current_hp(3.0),
            Unit::new(UnitType::Knight),
            Unit::new(UnitType::Catapult)
                .with_current_hp(5.0)
                .with_status_effects(StatusEffects::VETERAN),
            Unit::new(UnitType::Catapult),
        ]);
        let defenders = Units::from([Unit::new(UnitType::Giant)]);

        let (score, _log) = optimized(attackers, defenders);
        assert_eq!(score, 17.0);
    }

    #[test]
    fn test_b_ri_gi() {
        let attacker = Unit::new(UnitType::Rider);
        let defender = Unit::new(UnitType::Giant);

        let n_attacks = bulk(&attacker, defender);

        assert_eq!(n_attacks, 10);
    }

    #[test]
    fn test_b_sh_5_gi() {
        let attacker = Unit::new(UnitType::Shaman).with_current_hp(5.0);
        let defender = Unit::new(UnitType::Giant);

        let n_attacks = bulk(&attacker, defender);

        assert_eq!(n_attacks, 35);
    }

    #[test]
    fn test_e_wa_ri_u() {
        let attacker = Unit::new(UnitType::Warrior);
        let defender = Unit::new(UnitType::Rider);

        let max_hp = eliminate_defender(&attacker, defender);

        assert_eq!(max_hp, 7.0);
    }

    #[test]
    fn test_e_sh_1_gi_u() {
        let attacker = Unit::new(UnitType::Shaman).with_current_hp(1.0);
        let defender = Unit::new(UnitType::Giant);

        let max_hp = eliminate_defender(&attacker, defender);

        assert_eq!(max_hp, 2.0);
    }

    #[test]
    fn test_e_wa_u_ri_4() {
        let attacker = Unit::new(UnitType::Warrior);
        let defender = Unit::new(UnitType::Rider).with_current_hp(4.0);

        let min_hp = eliminate_attacker(attacker, &defender);

        assert_eq!(min_hp, 2.0);
    }

    #[test]
    fn test_e_gi_u_gi_15() {
        let attacker = Unit::new(UnitType::Giant);
        let defender = Unit::new(UnitType::Giant).with_current_hp(15.0);

        let min_hp = eliminate_attacker(attacker, &defender);

        assert_eq!(min_hp, 22.0);
    }
}
