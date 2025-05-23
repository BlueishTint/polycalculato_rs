use std::ops::Index;

use arrayvec::ArrayVec;
use generated::UNIT_TYPE_DATA;
use strum::IntoStaticStr;

mod generated;

const MAX_UNITS: usize = 12;

bitflags::bitflags! {
    /// A unit status effect.
    #[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
    pub struct StatusEffects: u16 {
        /// The unit's movement is increased by 1 and attack is increased by 0.5.
        const BOOSTED = 0b00000001;
        /// The unit is converted to the enemy's side.
        const CONVERTED = 0b00000010;
        /// The unit deals explosion damage to adjacent enemies.
        const EXPLODING = 0b00000100;
        /// The unit's defense is increased by 50%.
        const FORTIFIED = 0b00001000;
        /// The unit cannot use any actions.
        const FROZEN = 0b00010000;
        /// Reduces a unit's defense by 30% and removes all defense bonuses.
        const POISONED = 0b00100000;
        /// The unit deals splash damage to adjacent enemies.
        const SPLASHING = 0b01000000;
        /// The unit takes retaliation even if it has more range or the SURPRISE trait.
        const TAKES_RETALIATION = 0b10000000;
        /// The unit's max hp is increased by 1.
        const VETERAN = 0b00000001_00000000;
        /// The unit's defense is increased by 300%.
        const WALLED = 0b00000010_00000000;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, IntoStaticStr)]
pub enum UnitType {
    Archer,
    ArcherBomber,
    ArcherRaft,
    ArcherRammer,
    ArcherScout,
    BabyDragon,
    BattleSled,
    BattleSledBomber,
    BattleSledRaft,
    BattleSledRammer,
    BattleSledScout,
    Catapult,
    CatapultBomber,
    CatapultRaft,
    CatapultRammer,
    CatapultScout,
    Centipede,
    CentipedeBomber,
    CentipedeRaft,
    CentipedeRammer,
    CentipedeScout,
    Cloak,
    Crab,
    Dagger,
    DefaultWarrior,
    DefaultWarriorBomber,
    DefaultWarriorRaft,
    DefaultWarriorRammer,
    DefaultWarriorScout,
    Defender,
    DefenderBomber,
    DefenderRaft,
    DefenderRammer,
    DefenderScout,
    Doomux,
    DoomuxBomber,
    DoomuxRaft,
    DoomuxRammer,
    DoomuxScout,
    Egg,
    EggBomber,
    EggRaft,
    EggRammer,
    EggScout,
    Exida,
    ExidaBomber,
    ExidaRaft,
    ExidaRammer,
    ExidaScout,
    FireDragon,
    Gaami,
    GaamiBomber,
    GaamiRaft,
    GaamiRammer,
    GaamiScout,
    Giant,
    Hexapod,
    HexapodBomber,
    HexapodRaft,
    HexapodRammer,
    HexapodScout,
    IceArcher,
    IceArcherBomber,
    IceArcherRaft,
    IceArcherRammer,
    IceArcherScout,
    IceFortress,
    IceFortressBomber,
    IceFortressRaft,
    IceFortressRammer,
    IceFortressScout,
    Jelly,
    Juggernaut,
    Kiton,
    KitonBomber,
    KitonRaft,
    KitonRammer,
    KitonScout,
    Knight,
    KnightBomber,
    KnightRaft,
    KnightRammer,
    KnightScout,
    MindBender,
    MindBenderBomber,
    MindBenderRaft,
    MindBenderRammer,
    MindBenderScout,
    Mooni,
    MooniBomber,
    MooniRaft,
    MooniRammer,
    MooniScout,
    Phychi,
    Pirate,
    Polytaur,
    PolytaurBomber,
    PolytaurRaft,
    PolytaurRammer,
    PolytaurScout,
    Puffer,
    Raychi,
    Rider,
    RiderBomber,
    RiderRaft,
    RiderRammer,
    RiderScout,
    Segment,
    Shaman,
    ShamanBomber,
    ShamanRaft,
    ShamanRammer,
    ShamanScout,
    Shark,
    Swordsman,
    SwordsmanBomber,
    SwordsmanRaft,
    SwordsmanRammer,
    SwordsmanScout,
    Tridention,
    Warrior,
    WarriorBomber,
    WarriorRaft,
    WarriorRammer,
    WarriorScout,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub unit_type: UnitType,
    pub range: u8,
    pub status_effects: StatusEffects,
    pub trait_effects: StatusEffects,
    pub current_hp: f32,
    pub max_hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub defense_bonus: f32,
    pub retaliates: bool,
    pub surprise: bool,
}

impl Unit {
    pub fn new(unit_type: UnitType) -> Self {
        UNIT_TYPE_DATA[unit_type as usize].clone()
    }

    pub fn with_status_effects(mut self, status_effects: StatusEffects) -> Self {
        self.apply_status_effects(status_effects);
        self
    }

    pub fn with_current_hp(mut self, current_hp: f32) -> Self {
        self.current_hp = current_hp;
        self
    }

    #[inline]
    pub fn apply_status_effects(&mut self, status_effects: StatusEffects) {
        if status_effects.contains(StatusEffects::VETERAN)
            && !self.status_effects.contains(StatusEffects::VETERAN)
        {
            self.max_hp += 5.0;
        }

        self.status_effects.insert(status_effects);

        self.defense_bonus = if self.status_effects.contains(StatusEffects::POISONED) {
            0.7
        } else if self.status_effects.contains(StatusEffects::WALLED) {
            4.0
        } else if self.status_effects.contains(StatusEffects::FORTIFIED) {
            1.5
        } else {
            1.0
        };
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::new(UnitType::DefaultWarrior)
    }
}

#[derive(Debug, Clone)]
pub struct Units(ArrayVec<Unit, MAX_UNITS>);

impl Units {
    pub fn new() -> Self {
        Self { 0: ArrayVec::new() }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }
}

impl<const N: usize> From<[Unit; N]> for Units {
    fn from(src: [Unit; N]) -> Self {
        let mut out = Units::new();

        out.0.extend(src);

        out
    }
}

impl FromIterator<Unit> for Units {
    fn from_iter<I: IntoIterator<Item = Unit>>(iter: I) -> Self {
        let mut ret = Units::new();
        for (i, unit) in iter.into_iter().take(MAX_UNITS).enumerate() {
            ret.0[i] = unit;
        }
        ret
    }
}

impl Index<usize> for Units {
    type Output = Unit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Units {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
