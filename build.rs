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

fn main() {
    let yaml_str = std::fs::read_to_string("unit_data.yaml").unwrap();
    let raw_data: BTreeMap<String, RawUnitData> = serde_yaml::from_str(&yaml_str).unwrap();

    let mut out = String::new();
    out.push_str("// AUTO-GENERATED FILE, DO NOT EDIT\n");
    out.push_str("use crate::unit::{UnitTypeData, Trait};\n\n");
    out.push_str("pub static UNIT_TYPE_DATA: &[UnitTypeData] = &[\n");

    for (name, data) in raw_data {
        let traits_str = data
            .traits
            .iter()
            .map(|t| format!("Trait::{}", to_pascal_case(t)))
            .collect::<Vec<_>>()
            .join(", ");

        out.push_str(&format!(
            "    UnitTypeData {{ name: \"{}\", cost: {}, max_hp: {:?}, attack: {:?}, defense: {:?}, range: {}, traits: &[{}] }},\n",
            name, data.cost, data.hp, data.attack, data.defense, data.range, traits_str
        ));
    }

    out.push_str("];\n");

    std::fs::write("src/unit/generated.rs", out).unwrap();
}

/// Converts "fortify" -> "Fortify"
fn to_pascal_case(s: &str) -> String {
    s.split('_') // Split by underscores
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(f) => f.to_ascii_uppercase().to_string() + chars.as_str(), // Capitalize the first letter
                None => String::new(),
            }
        })
        .collect::<String>() // Join the words together
}
