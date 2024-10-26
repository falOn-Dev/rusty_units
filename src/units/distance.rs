use crate::dimension::Dimension;
use crate::{create_converters, create_dimension, create_operations};

#[derive(Debug, Clone, Copy)]
pub struct Distance {
    pub base_magnitude: f64,
}

create_converters!(Distance, meters => 1.0, feet => 3.28084, inches => 39.3701);
create_operations!(Distance);
create_dimension!(Distance);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_creation() {
        let dist_meters = Distance::from_meters(1.0);
        let dist_feet = Distance::from_feet(3.28084);
        let dist_inches = Distance::from_inches(39.3701);

        assert!((dist_meters.as_meters() - 1.0).abs() < f64::EPSILON);
        assert!((dist_feet.as_meters() - 1.0).abs() < f64::EPSILON);
        assert!((dist_inches.as_meters() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_conversion() {
        let dist_meters = Distance::from_meters(1.0);
        assert!((dist_meters.as_feet() - 3.28084).abs() < f64::EPSILON);
        assert!((dist_meters.as_inches() - 39.3701).abs() < f64::EPSILON);

        let dist_feet = Distance::from_feet(3.28084);
        assert!((dist_feet.as_meters() - 1.0).abs() < f64::EPSILON);
        assert!((dist_feet.as_inches() - 39.3701).abs() < f64::EPSILON);

        let dist_inches = Distance::from_inches(39.3701);
        assert!((dist_inches.as_meters() - 1.0).abs() < f64::EPSILON);
        assert!((dist_inches.as_feet() - 3.28084).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_addition() {
        let dist1 = Distance::from_meters(1.0);
        let dist2 = Distance::from_meters(2.0);
        let result = dist1 + dist2;
        assert!((result.as_meters() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_subtraction() {
        let dist1 = Distance::from_meters(3.0);
        let dist2 = Distance::from_meters(1.0);
        let result = dist1 - dist2;
        assert!((result.as_meters() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_multiplication() {
        let dist = Distance::from_meters(2.0);
        let result = dist * 3.0;
        assert!((result.as_meters() - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_distance_division() {
        let dist = Distance::from_meters(6.0);
        let result = dist / 2.0;
        assert_eq!(result.as_meters(), 3.0);
    }
}
