use rusty_units::units::{distance::Distance, time::Time};

fn main() {
    let velocity = Distance::from_feet(10.0) / Time::from_seconds(2.0);
    println!("Velocity (Furlongs/Fortnight): {}", velocity.as_furlongs_per_fortnight());
}