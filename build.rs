use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct RawUnitData {
    attack: f32,
    cost: u8,
    defense: f32,
    hp: f32,
    range: u8,
    traits: Vec<String>,
}

#[derive(Deserialize)]
struct RawNavalData {
    attack: f32,
    cost: u8,
    defense: f32,
    range: u8,
    traits: Vec<String>,
}

fn main() {
    let unit_str = std::fs::read_to_string("unit_data.yaml").unwrap();
    let raw_unit_data: BTreeMap<String, RawUnitData> = serde_yaml::from_str(&unit_str).unwrap();

    let mut out = String::new();
    out.push_str("// AUTO-GENERATED FILE, DO NOT EDIT\n");
    out.push_str("use crate::unit::{UnitTypeData, NavalTypeData, Traits};\n\n");
    out.push_str("pub static UNIT_TYPE_DATA: &[UnitTypeData] = &[\n");

    for (name, data) in raw_unit_data {
        let traits_str = data
            .traits
            .iter()
            .map(|t| format!("Traits::{}", t.to_uppercase()))
            .fold(String::new(), |acc, t| {
                if acc.is_empty() {
                    t
                } else {
                    format!("{}.union({})", acc, t)
                }
            });

        out.push_str(&format!(
            "    UnitTypeData {{ name: \"{}\", cost: {}, max_hp: {:?}, attack: {:?}, defense: {:?}, range: {}, traits: {} }},\n",
            name, data.cost, data.hp, data.attack, data.defense, data.range, traits_str
        ));
    }

    out.push_str("];\n\n");

    let naval_str = std::fs::read_to_string("naval_data.yaml").unwrap();
    let raw_naval_data: BTreeMap<String, RawNavalData> = serde_yaml::from_str(&naval_str).unwrap();

    out.push_str("pub static NAVAL_TYPE_DATA: &[NavalTypeData] = &[\n");

    for (name, data) in raw_naval_data {
        let traits_str = data
            .traits
            .iter()
            .map(|t| format!("Traits::{}", t.to_uppercase()))
            .fold(String::new(), |acc, t| {
                if acc.is_empty() {
                    t
                } else {
                    format!("{}.union({})", acc, t)
                }
            });

        out.push_str(&format!(
            "    NavalTypeData {{ name: \"{}\", cost: {}, attack: {:?}, defense: {:?}, range: {}, traits: {} }},\n",
                name, data.cost, data.attack, data.defense, data.range, traits_str
            ));
    }
    out.push_str("];\n");

    std::fs::write("src/unit/generated.rs", out).unwrap();
}
