# rusty_units

A simple an extensible library for working with physical units in rust.

Unit objects are differentiated by their "Dimension", ie distance, angle, linear velocity, etc. Each dimension has a base unit, this is usually the same as the SI unit of that dimension, so meters for length, radians for angle, etc.

To utilize the unit objects, you create an instance using your dimension of choice's `from_*` methods, where `*` can be any of your options for units of that dimension.

```rust
fn main() {
    let my_distance = Distance::from_feet(12.0);
}
```

All dimensions are stored in `f64` values, known as base magnitude, within the struct. And you can convert that value to any of the available expressisons of that dimensions using the `as_*` methods, where `*` can be any of your options for units of that dimension

```rust
fn main() {
    let my_distance = Distance::from_inches(12.0);
    println!(my_distance.as_feet()); // Prints 1.0
}
```

## Combining Units

Certain dimensions, like linear velocity, can be constructed by performing a mathematical operation on two other dimensions, in the case of linear velocity, this can be done with a distance, divided by a time.

```rust
fn main() {
    let dist = Distance::from_feet(10.0);
    let time = Time::from_seconds(2.0);

    let vel = dist / time

    println!(vel.as_feet_per_second()); // Prints 5.0
}
```

## Adding your own dimensions and operations

This library also provides some useful macros for creating new dimensions, and new relationships between those dimensions.

### Dimensions

To create a new dimension, you'll want to call the `create_unit!` macro, and then for the arguements pass in the name of the dimension, followed by as many units as you want, which will be input with `unit name => base unit divisor`. This will generate `from_*` methods, where the base magnitude gets initialized to `input / base unit divisor`, and `as_*` methods, where it returns `base_magnitude * base unit divisor`.

```rust
create_unit!(
    Distance, 
    meters => 1.0,
    feet => 3.28084,
    inches => 39.3701,
);

fn main() {
    let my_distance = Distance::from_inches(12.0);
    println!(my_distance.as_feet()); // Prints 1.0
}
```

### Operations

Unit operations are even easier to define, you simply define your 3 structs (lhs, rhs, and result), and then call the macro `create_operations!` and pass in `(lhs / rhs => result)` or `(lhs * rhs => result)`, to implement that operation trait onto the dimension passed in as the `lhs`.

```rust
create_unit_operations!(Distance / Time => LinearVelocity);

fn main() {
        let dist = Distance::from_feet(10.0);
    let time = Time::from_seconds(2.0);

    let vel = dist / time

    println!(vel.as_feet_per_second()); // Prints 5.0
}
```

As of right now, there is no way to add units to existing dimensions, this is likely to change at a later date.