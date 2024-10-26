use crate::{create_converters, create_dimension, create_operations, dimension::Dimension};


#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub base_magnitude: f64,
}

create_converters!(
    Time,
    seconds => 1.0,
    minutes => 1.0 / 60.0,
    hours => 1.0 / 3600.0,
    days => 1.0 / 86400.0,
    years => 1.0 / 31536000.0,
    fortnights => 1.0 / 1209600.0
);


create_operations!(Time);
create_dimension!(Time);

#[cfg(test)]
mod tests {
    #[test]
    fn test_time_creation() {
        let time_seconds = super::Time::from_seconds(1.0);
        let time_minutes = super::Time::from_minutes(1.0);
        let time_hours = super::Time::from_hours(1.0);
        let time_days = super::Time::from_days(1.0);
        let time_years = super::Time::from_years(1.0);
        let time_fortnights = super::Time::from_fortnights(1.0);

        assert!((time_seconds.as_seconds() - 1.0).abs() < f64::EPSILON);
        assert!((time_minutes.as_seconds() - 60.0).abs() < f64::EPSILON);
        assert!((time_hours.as_seconds() - 3600.0).abs() < f64::EPSILON);
        assert!((time_days.as_seconds() - 86400.0).abs() < f64::EPSILON);
        assert!((time_years.as_seconds() - 31536000.0).abs() < f64::EPSILON);
        assert!((time_fortnights.as_seconds() - 1209600.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_time_conversion() {
        let time_seconds = super::Time::from_seconds(3600.0);
        assert!((time_seconds.as_minutes() - 60.0).abs() < f64::EPSILON);
        assert!((time_seconds.as_hours() - 1.0).abs() < f64::EPSILON);
        assert!((time_seconds.as_days() - (1.0 / 24.0)).abs() < f64::EPSILON);
        assert!((time_seconds.as_years() - (1.0 / 8760.0)).abs() < f64::EPSILON);
        assert!((time_seconds.as_fortnights() - (1.0 / 336.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_time_addition() {
        let time1 = super::Time::from_seconds(3600.0);
        let time2 = super::Time::from_minutes(30.0);
        let result = time1 + time2;
        assert!((result.as_seconds() - 5400.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_time_subtraction() {
        let time1 = super::Time::from_hours(2.0);
        let time2 = super::Time::from_minutes(30.0);
        let result = time1 - time2;
        assert!((result.as_seconds() - 5400.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_time_multiplication() {
        let time = super::Time::from_minutes(30.0);
        let result = time * 2.0;
        assert!((result.as_seconds() - 3600.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_time_division() {
        let time = super::Time::from_hours(2.0);
        let result = time / 2.0;
        assert!((result.as_seconds() - 3600.0).abs() < f64::EPSILON);
    }
}
