pub mod generated;

bitflags::bitflags! {
    /// A unit trait
    #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
    pub struct Traits: u32 {
        /// Allows a unit to carry another unit inside.
        const CARRY = 0b00000000_00000000_00000000_00000001;
        /// Allows a unit to convert an enemy unit into a friendly unit by attacking it.
        const CONVERT = 0b00000000_00000000_00000000_00000010;
        /// Allows a unit to attack after moving in the same turn.
        const DASH = 0b00000000_00000000_00000000_00000100;
        /// Allows a unit to move after attacking in the same turn.
        const ESCAPE = 0b00000000_00000000_00000000_00001000;
        /// Allows a unit to receive a defence bonus in a city.
        const FORTIFY = 0b00000000_00000000_00000000_00010000;
        /// Gives a unit the Heal Others unit action, which heals all adjacent friendly units by up to 4 HP.
        const HEAL = 0b00000000_00000000_00000000_00100000;
        /// Allows a unit to attack again immediately after killing an enemy unit.
        const PERSIST = 0b00000000_00000000_00000000_01000000;
        /// Allows a unit to explore a 5x5 area instead of a 3x3 area.
        const SCOUT = 0b00000000_00000000_00000000_10000000;
        /// Allows a unit to damage or poison enemy units adjacent to the targeted unit.
        const SPLASH = 0b00000000_00000001_00000000_00000000;
        /// Prevents a unit from becoming a veteran.
        const STATIC = 0b00000000_00000010_00000000_00000000;
        /// Prevents a unit from retaliating when attacked by an enemy unit.
        const STIFF = 0b00000000_00000100_00000000_00000000;
        /// Causes a unit to deal damage to all adjacent enemy units when it moves.
        const STOMP = 0b00000000_00001000_00000000_00000000;
        /// Prevents a unit from triggering retaliation attacks when attacking an enemy unit.
        const SURPRISE = 0b00000000_00010000_00000000_00000000;
        /// Allows a unit to ignore movement barriers imposed by terrain.
        const CREEP = 0b00000000_00100000_00000000_00000000;
        /// Allows a unit to hide itself and become invisible to enemies when it moves.
        const HIDE = 0b00000000_01000000_00000000_00000000;
        /// Allows a unit to incite a revolt and spawn Daggers by entering an enemy city.
        const INFILTRATE = 0b00000000_10000000_00000000_00000000;
        /// Allows a unit to automatically flood any tile it moves onto.
        const AUTOFLOOD = 0b00000001_00000000_00000000_00000000;
        /// Allows a unit to flood any tile it attacks.
        const DRENCH = 0b00000010_00000000_00000000_00000000;
        /// Allows a unit to damage any enemy that moves next to it, as well as when it moves next to an enemy or when it is trained.
        const TENTACLES = 0b00000100_00000000_00000000_00000000;
        /// Allows a unit to grow into a different unit after a given number of turns.
        const GROW = 0b00001000_00000000_00000000_00000000;
        /// Units with this skill do not take up a population slot in or belong to any city.
        const INDEPENDENT = 0b00010000_00000000_00000000_00000000;
        /// Allows a unit to automatically freeze adjacent enemy units and water tiles (turning them into ice tiles) as it moves.
        const AUTO_FREEZE = 0b00100000_00000000_00000000_00000000;
        /// Allows a unit to freeze enemy units it attacks.
        const FREEZE = 0b01000000_00000000_00000000_00000000;
        /// Gives a unit the Freeze Area unit action, which freezes adjacent enemy units, freezes adjacent water tiles into ice tiles, and converts adjacent land tiles to the style of the tribe the unit belongs to.
        const FREEZE_AREA = 0b10000000_00000000_00000000_00000000;
        /// Doubles movement on ice but limits movement to one and prohibits the use of the dash and escape skills on land.
        const SKATE = 0b00000000_00000000_00000001_00000000;
        /// Gives a unit the Boost unit action, which boosts all adjacent friendly units by increasing their attack by 0.5 and movement by 1 until their next action (excluding moving).
        const BOOST = 0b00000000_00000000_00000010_00000000;
        /// Allows a unit to grow in length for every kill.
        const EAT = 0b00000000_00000000_00000100_00000000;
        /// Gives a unit the Explode unit action, which damages using the unit's attack value and poisons all adjacent enemy units, kills the unit itself, and leaves in its place spores (on land) or Algae (on water).
        const EXPLODE = 0b00000000_00000000_00001000_00000000;
        /// Allows a unit to move in ocean even if no prerequisite technology is researched but prevents the unit from moving onto land, except for capturing cities and villages.
        const NAVIGATE = 0b00000000_00000000_00010000_00000000;
        /// Allows a unit to poison enemy units it attacks.
        const POISON = 0b00000000_00000000_00100000_00000000;
        /// Allows a unit to ignore movement barriers imposed by enemy units.
        const SNEAK = 0b00000000_00000000_01000000_00000000;
    }
}

bitflags::bitflags! {
    /// A unit status effect.
    #[derive(PartialEq, Eq, Hash, Debug)]
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

pub struct UnitTypeData {
    pub name: &'static str,
    pub cost: u8,
    pub max_hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub range: u8,
    pub traits: Traits,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum UnitType {
    Archer,
    BabyDragon,
    BattleSled,
    Catapult,
    Centipede,
    Cloak,
    Crab,
    Dagger,
    DefaultWarrior,
    Defender,
    Doomux,
    Egg,
    Exida,
    FireDragon,
    Gaami,
    Giant,
    Hexapod,
    IceArcher,
    IceFortress,
    Jelly,
    Juggernaut,
    Kiton,
    Knight,
    MindBender,
    Mooni,
    Phychi,
    Pirate,
    Polytaur,
    Puffer,
    Raychi,
    Rider,
    Segment,
    Shaman,
    Shark,
    Swordsman,
    Tridention,
    Warrior,
}

pub struct Unit {
    unit_type: UnitType,
    pub current_hp: f32,
    pub status_effects: StatusEffects,
}

impl Unit {
    pub fn new(
        unit_type: UnitType,
        current_hp: Option<f32>,
        status_effects: Option<StatusEffects>,
    ) -> Self {
        Unit {
            unit_type,
            current_hp: current_hp.unwrap_or(generated::UNIT_TYPE_DATA[unit_type as usize].max_hp),
            status_effects: status_effects.unwrap_or(StatusEffects::empty()),
        }
    }

    #[inline]
    pub fn attack(&self) -> f32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].attack
    }

    #[inline]
    pub fn cost(&self) -> u8 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].cost
    }

    #[inline]
    pub fn defense(&self) -> f32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].defense
    }

    #[inline]
    pub fn max_hp(&self) -> f32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].max_hp
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].name
    }

    #[inline]
    pub fn range(&self) -> u8 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].range
    }

    #[inline]
    pub fn traits(&self) -> Traits {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].traits
    }

    #[inline]
    pub fn health_ratio(&self) -> f32 {
        self.current_hp / self.max_hp()
    }

    pub fn defense_bonus(&self) -> f32 {
        if self.status_effects.contains(StatusEffects::POISONED) {
            0.7
        } else if self.status_effects.contains(StatusEffects::WALLED) {
            3.0
        } else if self.status_effects.contains(StatusEffects::FORTIFIED) {
            1.5
        } else {
            1.0
        }
    }

    pub fn archer() -> Self {
        Unit::new(UnitType::Archer, None, None)
    }

    pub fn baby_dragon() -> Self {
        Unit::new(UnitType::BabyDragon, None, None)
    }

    pub fn battle_sled() -> Self {
        Unit::new(UnitType::BattleSled, None, None)
    }

    pub fn catapult() -> Self {
        Unit::new(UnitType::Catapult, None, None)
    }

    pub fn centipede() -> Self {
        Unit::new(UnitType::Centipede, None, None)
    }

    pub fn cloak() -> Self {
        Unit::new(UnitType::Cloak, None, None)
    }

    pub fn crab() -> Self {
        Unit::new(UnitType::Crab, None, None)
    }

    pub fn dagger() -> Self {
        Unit::new(UnitType::Dagger, None, None)
    }

    pub fn defender() -> Self {
        Unit::new(UnitType::Defender, None, None)
    }

    pub fn doomux() -> Self {
        Unit::new(UnitType::Doomux, None, None)
    }

    pub fn egg() -> Self {
        Unit::new(UnitType::Egg, None, None)
    }

    pub fn exida() -> Self {
        Unit::new(UnitType::Exida, None, None)
    }

    pub fn fire_dragon() -> Self {
        Unit::new(UnitType::FireDragon, None, None)
    }

    pub fn gaami() -> Self {
        Unit::new(UnitType::Gaami, None, None)
    }

    pub fn giant() -> Self {
        Unit::new(UnitType::Giant, None, None)
    }

    pub fn hexapod() -> Self {
        Unit::new(UnitType::Hexapod, None, None)
    }

    pub fn ice_archer() -> Self {
        Unit::new(UnitType::IceArcher, None, None)
    }

    pub fn ice_fortress() -> Self {
        Unit::new(UnitType::IceFortress, None, None)
    }

    pub fn jelly() -> Self {
        Unit::new(UnitType::Jelly, None, None)
    }

    pub fn juggernaut() -> Self {
        Unit::new(UnitType::Juggernaut, None, None)
    }

    pub fn kiton() -> Self {
        Unit::new(UnitType::Kiton, None, None)
    }

    pub fn knight() -> Self {
        Unit::new(UnitType::Knight, None, None)
    }

    pub fn mind_bender() -> Self {
        Unit::new(UnitType::MindBender, None, None)
    }

    pub fn mooni() -> Self {
        Unit::new(UnitType::Mooni, None, None)
    }

    pub fn phychi() -> Self {
        Unit::new(UnitType::Phychi, None, None)
    }

    pub fn pirate() -> Self {
        Unit::new(UnitType::Pirate, None, None)
    }

    pub fn polytaur() -> Self {
        Unit::new(UnitType::Polytaur, None, None)
    }

    pub fn puffer() -> Self {
        Unit::new(UnitType::Puffer, None, None)
    }

    pub fn raychi() -> Self {
        Unit::new(UnitType::Raychi, None, None)
    }

    pub fn rider() -> Self {
        Unit::new(UnitType::Rider, None, None)
    }

    pub fn segment() -> Self {
        Unit::new(UnitType::Segment, None, None)
    }

    pub fn shaman() -> Self {
        Unit::new(UnitType::Shaman, None, None)
    }

    pub fn shark() -> Self {
        Unit::new(UnitType::Shark, None, None)
    }

    pub fn swordsman() -> Self {
        Unit::new(UnitType::Swordsman, None, None)
    }

    pub fn tridention() -> Self {
        Unit::new(UnitType::Tridention, None, None)
    }

    pub fn warrior() -> Self {
        Unit::new(UnitType::Warrior, None, None)
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            unit_type: UnitType::DefaultWarrior,
            current_hp: 10.0,
            status_effects: StatusEffects::empty(),
        }
    }
}
