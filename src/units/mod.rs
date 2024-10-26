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

