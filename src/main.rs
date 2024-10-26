use rusty_units::units::LinearVelocity;

fn main() {
    let velocity = LinearVelocity::from_feet_per_second(1.0);
    println!("Velocity: {} ft/s", velocity.as_feet_per_second());
}