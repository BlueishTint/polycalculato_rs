use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
struct RawUnitData {
    attack: f32,
    cost: u8,
    defense: f32,
    hp: f32,
    range: u8,
    retaliates: bool,
    surprise: bool,
    trait_effects: Option<String>,
}

fn main() {
    let unit_str = std::fs::read_to_string("unit_data.yaml").unwrap();
    let raw_unit_data: BTreeMap<String, RawUnitData> = serde_yaml::from_str(&unit_str).unwrap();

    let mut out = String::new();
    out.push_str("// AUTO-GENERATED FILE, DO NOT EDIT\n");
    out.push_str("use crate::unit::{StatusEffects, Unit};\n\n");
    out.push_str("pub const UNIT_TYPE_DATA: &'static [Unit] = &[\n");

    for (name, data) in raw_unit_data {
        if let Some(trait_effects) = data.trait_effects {
            out.push_str(&format!("    Unit {{ attack: {:?}, current_hp: {:?}, defense: {:?}, max_hp: {:?}, range: {}, retaliates: {}, status_effects: StatusEffects::empty(), surprise: {}, trait_effects: StatusEffects::{}, defense_bonus: 1.0 }}, // {}\n", data.attack, data.hp, data.defense, data.hp, data.range, data.retaliates, data.surprise, trait_effects, name));
        } else {
            out.push_str(&format!("    Unit {{ attack: {:?}, current_hp: {:?}, defense: {:?}, max_hp: {:?}, range: {}, retaliates: {}, status_effects: StatusEffects::empty(), surprise: {}, trait_effects: StatusEffects::empty(), defense_bonus: 1.0 }}, // {}\n", data.attack, data.hp, data.defense, data.hp, data.range, data.retaliates, data.surprise, name));
        }
        match name.as_str() {
            "BabyDragon" | "Cloak" | "Crab" | "Dagger" | "FireDragon" | "Giant" | "Jelly"
            | "Juggernaut" | "Phychi" | "Pirate" | "Puffer" | "Raychi" | "Segment" | "Shark"
            | "Tridention" => {}
            _ => {
                out.push_str(&format!("    Unit {{ attack: {:?}, current_hp: {:?}, defense: {:?}, max_hp: {:?}, range: {}, retaliates: {}, status_effects: StatusEffects::empty(), surprise: {}, trait_effects: StatusEffects::empty(), defense_bonus: 1.0 }}, // {}Bomber\n", 3.0, data.hp, 2.0, data.hp, 3, false, false, name));
                out.push_str(&format!("    Unit {{ attack: {:?}, current_hp: {:?}, defense: {:?}, max_hp: {:?}, range: {}, retaliates: {}, status_effects: StatusEffects::empty(), surprise: {}, trait_effects: StatusEffects::empty(), defense_bonus: 1.0 }}, // {}Raft\n", 0.0, data.hp, 2.0, data.hp, 0, false, false, name));
                out.push_str(&format!("    Unit {{ attack: {:?}, current_hp: {:?}, defense: {:?}, max_hp: {:?}, range: {}, retaliates: {}, status_effects: StatusEffects::empty(), surprise: {}, trait_effects: StatusEffects::empty(), defense_bonus: 1.0 }}, // {}Rammer\n", 3.0, data.hp, 3.0, data.hp, 1, true, false, name));
                out.push_str(&format!("    Unit {{ attack: {:?}, current_hp: {:?}, defense: {:?}, max_hp: {:?}, range: {}, retaliates: {}, status_effects: StatusEffects::empty(), surprise: {}, trait_effects: StatusEffects::empty(), defense_bonus: 1.0 }}, // {}Scout\n", 2.0, data.hp, 1.0, data.hp, 2, true, false, name));
            }
        }
    }

    out.push_str("];\n");

    std::fs::write("./src/unit/generated.rs", out).expect("Unable to write to generated.rs");
}
