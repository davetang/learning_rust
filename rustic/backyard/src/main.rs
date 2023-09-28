use crate::garden::vegetables::Asparagus;

// Declare new garden module, which is in src/garden.rs.
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
