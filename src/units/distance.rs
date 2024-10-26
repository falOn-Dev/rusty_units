use crate::dimension::Dimension;
use crate::{create_converters, create_dimension, create_operations};


#[derive(Debug, Clone, Copy)]
pub struct Distance {
    pub base_magnitude: f64,
}

create_converters!(Distance, meters => 1.0, feet => 3.28084, inches => 39.3701);
create_operations!(Distance);
create_dimension!(Distance);
