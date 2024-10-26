use crate::{create_converters, create_dimension, create_operations, dimension::Dimension};


#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub base_magnitude: f64,
}

create_converters!(Time, seconds => 1.0, minutes => 1.0 / 60.0, hours => 1.0 / 3600.0);
create_operations!(Time);
create_dimension!(Time);