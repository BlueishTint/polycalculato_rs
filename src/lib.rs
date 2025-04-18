#![allow(dead_code)]

pub mod combat;
pub mod unit;

fn main() {
    let warrior = unit::Unit::warrior();
    let archer = unit::Unit::archer();

    let combat = combat::single_combat(&warrior, &archer);

    println!("Combat result: {:?}", combat);
}
