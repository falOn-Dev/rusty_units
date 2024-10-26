use linear_velocity::LinearVelocity;
use time::Time;
use distance::Distance;

use crate::create_unit_operations;

pub mod distance;
pub mod time;
pub mod linear_velocity;

create_unit_operations!(
    Distance / Time => LinearVelocity
);