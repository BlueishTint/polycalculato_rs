pub mod generated;


/// A unit trait
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Trait {
    /// Allows a unit to carry another unit inside.
    /// A unit with the carry skill can move to a land tile adjacent to water.
    /// Doing so releases the unit it was carrying and ends the unit's turn.
    Carry,
    /// Allows a unit to convert an enemy unit into a friendly unit by attacking it.
    Convert,
    /// Allows a unit to attack after moving in the same turn.
    Dash,
    /// Allows a unit to move after attacking in the same turn.
    Escape,
    /// Allows a unit to receive a defence bonus in a city.
    Fortify,
    /// Gives a unit the Heal Others unit action, which
    /// heals all adjacent friendly units by up to 4 HP.
    Heal,
    /// Allows a unit to attack again immediately after killing an enemy unit.
    /// There is no limit on the number of kills in a single turn.
    Persist,
    /// Allows a unit to explore a 5x5 area instead of a 3x3 area.
    Scout,
    /// Allows a unit to damage or poison enemy units adjacent to the targeted unit.
    Splash,
    /// Prevents a unit from becoming a veteran.
    Static,
    /// Prevents a unit from retaliating when attacked by an enemy unit.
    Stiff,
    /// Causes a unit to deal damage to all adjacent enemy units when it moves.
    Stomp,
    /// Prevents a unit from triggering retaliation attacks when attacking an enemy unit.
    Surprise,
    /// Allows a unit to ignore movement barriers imposed by terrain.
    Creep,
    /// Allows a unit to hide itself and become invisible to enemies when it moves.
    Hide,
    /// Allows a unit to incite a revolt and spawn Daggers by entering an enemy city.
    Infiltrate,
    /// Allows a unit to automatically flood any tile it moves onto.
    Autoflood,
    /// Allows a unit to flood any tile it attacks.
    Drench,
    /// Allows a unit to damage any enemy that moves next to it,
    /// as well as when it moves next to an enemy or when it is trained.
    Tentacles,
    /// Allows a unit to grow into a different unit after a given number of turns.
    Grow,
    /// Units with this skill do not take up a population slot in or belong to any city.
    Independent,
    /// Allows a unit to automatically freeze adjacent enemy units and water tiles
    /// (turning them into ice tiles) as it moves.
    AutoFreeze,
    /// Allows a unit to freeze enemy units it attacks.
    Freeze,
    /// Gives a unit the Freeze Area unit action, which freezes adjacent enemy units,
    /// freezes adjacent water tiles into ice tiles, and converts adjacent land tiles to the
    /// style of the tribe the unit belongs to.
    FreezeArea,
    /// Doubles movement on ice but limits movement to one and prohibits the use of the
    /// dash and escape skills on land.
    Skate,
    /// Gives a unit the Boost unit action, which boosts all adjacent friendly units by
    /// increasing their attack by 0.5 and movement by 1 until their next action
    /// (excluding moving).
    Boost,
    /// Allows a unit to grow in length for every kill.
    Eat,
    /// Gives a unit the Explode unit action, which damages using the unit's attack value
    /// and poisons all adjacent enemy units, kills the unit itself, and leaves in its place
    /// spores (on land) or Algae (on water).
    Explode,
    /// Allows a unit to move in ocean even if no prerequisite
    /// technology is researched but prevents the unit from moving
    /// onto land, except for capturing cities and villages.
    Navigate,
    /// Allows a unit to poison enemy units it attacks.
    Poison,
    /// Allows a unit to ignore movement barriers imposed by enemy units.
    Sneak,
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
    pub cost: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub range: u32,
    pub traits: &'static [Trait],
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
    pub current_hp: u32,
    pub status_effects: StatusEffects,
}

impl Unit {
    pub fn new(unit_type: UnitType, current_hp: Option<u32>, status_effects: Option<StatusEffects>) -> Self {
        Unit {
            unit_type,
            current_hp: current_hp.unwrap_or(100),
            status_effects: status_effects.unwrap_or(StatusEffects::empty()),
        }
    }

    #[inline(always)]
    pub fn attack(&self) -> u32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].attack
    }

    #[inline(always)]
    pub fn cost(&self) -> u32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].cost
    }

    #[inline(always)]
    pub fn defense(&self) -> u32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].defense
    }
    #[inline(always)]
    pub fn max_hp(&self) -> u32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].max_hp
    }

    #[inline(always)]
    pub fn name(&self) -> &'static str {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].name
    }

    #[inline(always)]
    pub fn range(&self) -> u32 {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].range
    }

    #[inline(always)]
    pub fn traits(&self) -> &'static [Trait] {
        generated::UNIT_TYPE_DATA[self.unit_type as usize].traits
    }

    #[inline(always)]
    pub fn health_ratio(&self) -> f32 {
        self.current_hp as f32 / self.max_hp() as f32
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            unit_type: UnitType::DefaultWarrior,
            current_hp: 100,
            status_effects: StatusEffects::empty(),
        }
    }
}