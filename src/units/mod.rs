use crate::{create_unit_operations, create_unit};


create_unit!(
    Distance, 
    meters => 1.0,
    feet => 3.28084,
    inches => 39.3701,
    miles => 0.000621371,
    kilometers => 0.001,
    nautical_miles => 0.000539957,
    furlongs => 0.00497096
);

create_unit!(
    Time,
    seconds => 1.0,
    minutes => 1.0 / 60.0,
    hours => 1.0 / 3600.0,
    days => 1.0 / 86400.0,
    years => 1.0 / 31536000.0,
    fortnights => 1.0 / 1209600.0
);

create_unit!(
    LinearVelocity,
    meters_per_second => 1.0,
    feet_per_second => 3.28084,
    inches_per_second => 39.3701,
    miles_per_hour => 2.23694,
    kilometers_per_hour => 3.6,
    knots => 1.94384,
    furlongs_per_fortnight => 6012.87
);

create_unit!(
    Angle,
    radians => 1.0,
    rotations => 0.159155,
    degrees => 180.0 / std::f64::consts::PI,
    gradians => 200.0 / std::f64::consts::PI,
    arcminutes => 60.0 * 180.0 / std::f64::consts::PI
);

create_unit!(
    AngularVelocity,
    radians_per_second => 1.0,
    rotations_per_second => 0.159155,
    rotations_per_minute => 0.159155 * 60.0,
    degrees_per_second => 180.0 / std::f64::consts::PI,
    gradians_per_second => 200.0 / std::f64::consts::PI,
    arcminutes_per_second => 60.0 * 180.0 / std::f64::consts::PI
);

create_unit!(
    Mass,
    kilograms => 1.0,
    grams => 1000.0,
    pounds => 2.20462,
    ounces => 35.274,
    stones => 0.157473,
    tons => 0.001
);

create_unit!(
    Force,
    newtons => 1.0,
    pounds_force => 0.224809,
    dynes => 100000.0
);

create_unit!(
    Torque,
    newton_meters => 1.0,
    pound_feet => 0.737562,
    pound_inches => 8.85075
);

create_unit!(
    Temperature,
    kelvin => 1.0,
    celsius => 1.0,
    fahrenheit => 1.8
);

create_unit!(
    Energy,
    joules => 1.0,
    kilojoules => 0.001,
    calories => 0.239006,
    kilocalories => 0.000239006,
    watt_hours => 0.000277778,
    kilowatt_hours => 2.77778e-7
);

create_unit!(
    Power,
    watts => 1.0,
    kilowatts => 0.001,
    horsepower => 0.00134102
);

create_unit!(
    Pressure,
    pascals => 1.0,
    kilopascals => 0.001,
    bar => 1e-5,
    psi => 0.000145038,
    atmospheres => 9.86923e-6
);

create_unit!(
    ElectricPotential,
    volts => 1.0,
    millivolts => 1000.0,
    microvolts => 1_000_000.0
);

create_unit!(
    Current,
    amperes => 1.0,
    milliamperes => 1000.0,
    microamperes => 1_000_000.0
);


create_unit_operations!(Distance / Time => LinearVelocity);
create_unit_operations!(Angle / Time => AngularVelocity);
create_unit_operations!(Force * Distance => Torque);
create_unit_operations!(Energy / Time => Power);
create_unit_operations!(Power / ElectricPotential => Current);
create_unit_operations!(Power / Current => ElectricPotential);
create_unit_operations!(ElectricPotential * Current => Power);


#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_distance_conversion() {
        let meters = Distance::from_meters(1.0);
        assert!((meters.as_feet() - 3.28084).abs() < EPSILON);
        assert!((meters.as_inches() - 39.3701).abs() < EPSILON);
        assert!((meters.as_miles() - 0.000621371).abs() < EPSILON);
        assert!((meters.as_kilometers() - 0.001).abs() < EPSILON);
        assert!((meters.as_nautical_miles() - 0.000539957).abs() < EPSILON);
        assert!((meters.as_furlongs() - 0.00497096).abs() < EPSILON);
    }

    #[test]
    fn test_time_conversion() {
        let seconds = Time::from_seconds(1.0);
        assert!((seconds.as_minutes() - 1.0 / 60.0).abs() < EPSILON);
        assert!((seconds.as_hours() - 1.0 / 3600.0).abs() < EPSILON);
        assert!((seconds.as_days() - 1.0 / 86400.0).abs() < EPSILON);
        assert!((seconds.as_years() - 1.0 / 31536000.0).abs() < EPSILON);
        assert!((seconds.as_fortnights() - 1.0 / 1209600.0).abs() < EPSILON);
    }

    #[test]
    fn test_linear_velocity_conversion() {
        let mps = LinearVelocity::from_meters_per_second(1.0);
        assert!((mps.as_feet_per_second() - 3.28084).abs() < EPSILON);
        assert!((mps.as_inches_per_second() - 39.3701).abs() < EPSILON);
        assert!((mps.as_miles_per_hour() - 2.23694).abs() < EPSILON);
        assert!((mps.as_kilometers_per_hour() - 3.6).abs() < EPSILON);
        assert!((mps.as_knots() - 1.94384).abs() < EPSILON);
        assert!((mps.as_furlongs_per_fortnight() - 6012.87).abs() < EPSILON);
    }

    #[test]
    fn test_angle_conversion() {
        let radians = Angle::from_radians(1.0);
        assert!((radians.as_rotations() - 0.159155).abs() < EPSILON);
        assert!((radians.as_degrees() - (180.0 / std::f64::consts::PI)).abs() < EPSILON);
        assert!((radians.as_gradians() - (200.0 / std::f64::consts::PI)).abs() < EPSILON);
        assert!((radians.as_arcminutes() - (60.0 * 180.0 / std::f64::consts::PI)).abs() < EPSILON);
    }

    #[test]
    fn test_angular_velocity_conversion() {
        let rps = AngularVelocity::from_radians_per_second(1.0);
        assert!((rps.as_rotations_per_second() - 0.159155).abs() < EPSILON);
        assert!((rps.as_rotations_per_minute() - (0.159155 * 60.0)).abs() < EPSILON);
        assert!((rps.as_degrees_per_second() - (180.0 / std::f64::consts::PI)).abs() < EPSILON);
        assert!((rps.as_gradians_per_second() - (200.0 / std::f64::consts::PI)).abs() < EPSILON);
        assert!((rps.as_arcminutes_per_second() - (60.0 * 180.0 / std::f64::consts::PI)).abs() < EPSILON);
    }

    #[test]
    fn test_mass_conversion() {
        let kg = Mass::from_kilograms(1.0);
        assert!((kg.as_grams() - 1000.0).abs() < EPSILON);
        assert!((kg.as_pounds() - 2.20462).abs() < EPSILON);
        assert!((kg.as_ounces() - 35.274).abs() < EPSILON);
        assert!((kg.as_stones() - 0.157473).abs() < EPSILON);
        assert!((kg.as_tons() - 0.001).abs() < EPSILON);
    }

    #[test]
    fn test_force_conversion() {
        let newtons = Force::from_newtons(1.0);
        assert!((newtons.as_pounds_force() - 0.224809).abs() < EPSILON);
        assert!((newtons.as_dynes() - 100000.0).abs() < EPSILON);
    }

    #[test]
    fn test_torque_conversion() {
        let nm = Torque::from_newton_meters(1.0);
        assert!((nm.as_pound_feet() - 0.737562).abs() < EPSILON);
        assert!((nm.as_pound_inches() - 8.85075).abs() < EPSILON);
    }

    #[test]
    fn test_temperature_conversion() {
        let kelvin = Temperature::from_kelvin(1.0);
        assert!((kelvin.as_celsius() - 1.0).abs() < EPSILON);
        assert!((kelvin.as_fahrenheit() - 1.8).abs() < EPSILON);
    }

    #[test]
    fn test_energy_conversion() {
        let joules = Energy::from_joules(1.0);
        assert!((joules.as_kilojoules() - 0.001).abs() < EPSILON);
        assert!((joules.as_calories() - 0.239006).abs() < EPSILON);
        assert!((joules.as_kilocalories() - 0.000239006).abs() < EPSILON);
        assert!((joules.as_watt_hours() - 0.000277778).abs() < EPSILON);
        assert!((joules.as_kilowatt_hours() - 2.77778e-7).abs() < EPSILON);
    }

    #[test]
    fn test_power_conversion() {
        let watts = Power::from_watts(1.0);
        assert!((watts.as_kilowatts() - 0.001).abs() < EPSILON);
        assert!((watts.as_horsepower() - 0.00134102).abs() < EPSILON);
    }

    #[test]
    fn test_pressure_conversion() {
        let pascals = Pressure::from_pascals(1.0);
        assert!((pascals.as_kilopascals() - 0.001).abs() < EPSILON);
        assert!((pascals.as_bar() - 1e-5).abs() < EPSILON);
        assert!((pascals.as_psi() - 0.000145038).abs() < EPSILON);
        assert!((pascals.as_atmospheres() - 9.86923e-6).abs() < EPSILON);
    }

    #[test]
    fn test_electric_potential_conversion() {
        let volts = ElectricPotential::from_volts(1.0);
        assert!((volts.as_millivolts() - 1000.0).abs() < EPSILON);
        assert!((volts.as_microvolts() - 1_000_000.0).abs() < EPSILON);
    }

    #[test]
    fn test_current_conversion() {
        let amperes = Current::from_amperes(1.0);
        assert!((amperes.as_milliamperes() - 1000.0).abs() < EPSILON);
        assert!((amperes.as_microamperes() - 1_000_000.0).abs() < EPSILON);
    }
}

