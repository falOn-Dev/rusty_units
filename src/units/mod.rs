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
    degrees_per_second => 180.0 / std::f64::consts::PI,
    gradians_per_second => 200.0 / std::f64::consts::PI,
    arcminutes_per_second => 60.0 * 180.0 / std::f64::consts::PI
);

