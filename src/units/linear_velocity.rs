use crate::{create_converters, create_dimension, create_operations};

#[derive(Debug, Clone, Copy)]
pub struct LinearVelocity {
    pub base_magnitude: f64,
}

create_converters!(
    LinearVelocity,
    meters_per_second => 1.0,
    feet_per_second => 3.28084,
    inches_per_second => 39.3701,
    miles_per_hour => 2.23694,
    kilometers_per_hour => 3.6,
    knots => 1.94384,
    furlongs_per_fortnight => 6012.87
);

create_operations!(LinearVelocity);
create_dimension!(LinearVelocity);
