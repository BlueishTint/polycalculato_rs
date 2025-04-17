mod combat;
mod unit;

fn main() {
    // Example usage of the Unit struct
    let mut unit = unit::Unit::raychi();
    unit.current_hp = 90.0;
    println!("Unit type: {:?}", unit.name());
    println!("Max HP: {}", unit.max_hp());
    println!("Current HP: {}", unit.current_hp);
    println!("Status Effects: {:?}", unit.status_effects);
    println!("Traits: {:?}", unit.traits());
}
