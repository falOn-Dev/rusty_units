//! This module provides a set of macros to create unit conversion structures and their associated operations.
//!
//! # Macros
//!
//! ## `create_converters`
//! 
//! This macro generates conversion methods for a given unit structure. It takes a structure name and a list of unit names with their conversion factors.
//!
//! ### Example
//!
//! ```rust
//! create_converters!(Length, meter => 1.0, kilometer => 1000.0);
//! ```
//!
//! This will generate methods like `as_meter`, `from_meter`, `as_kilometer`, and `from_kilometer` for the `Length` struct.
//!
//! ## `create_operations`
//!
//! This macro generates basic arithmetic operations (`Add`, `Sub`, `Mul`, `Div`) for a given unit structure. It also implements `PartialEq` for the structure.
//!
//! ### Example
//!
//! ```rust
//! create_operations!(Length);
//! ```
//!
//! This will enable addition, subtraction, multiplication by a scalar, and division by a scalar for the `Length` struct.
//!
//! ## `create_dimension`
//!
//! This macro implements the `Dimension` trait for a given unit structure. The `Dimension` trait requires methods to convert to and from base units.
//!
//! ### Example
//!
//! ```rust
//! create_dimension!(Length);
//! ```
//!
//! This will implement the `Dimension` trait for the `Length` struct, providing methods `as_base_units` and `from_base_units`.
//!
//! ## `create_unit_operations`
//!
//! This macro generates multiplication and division operations between two different unit structures, resulting in a third unit structure.
//!
//! ### Example
//!
//! ```rust
//! create_unit_operations!(Length * Time => Speed);
//! create_unit_operations!(Length / Time => Speed);
//! ```
//!
//! This will enable multiplication and division between `Length` and `Time` structs, resulting in a `Speed` struct.
//!
//! ## `create_unit`
//!
//! This macro combines the functionality of `create_converters`, `create_operations`, and `create_dimension` to create a complete unit structure with all necessary methods and traits.
//!
//! ### Example
//!
//! ```rust
//! create_unit!(Length, meter => 1.0, kilometer => 1000.0);
//! ```
//!
//! This will generate a `Length` struct with conversion methods, arithmetic operations, and the `Dimension` trait implementation.
#[macro_export]
macro_rules! create_converters {
    ($struct_name:ident, $( $unit_name:ident => $conversion_factor:expr ),+ ) => {
        paste::paste!{
        impl $struct_name {
            $(
                pub fn [< as_ $unit_name >](&self) -> f64 {
                    self.0 * $conversion_factor
                }

                pub fn [< from_ $unit_name >](value: f64) -> Self {
                    $struct_name(value / $conversion_factor)
                }
            )+
        }
        }
    };
}

#[macro_export]
/// This macro generates common arithmetic and comparison operations for a given struct.
/// 
/// # Parameters
/// - `$struct_name`: The name of the struct for which the operations will be implemented.
/// 
/// # Generated Implementations
/// - `std::ops::Add`: Adds two instances of the struct.
/// - `std::ops::Sub`: Subtracts one instance of the struct from another.
/// - `std::ops::Mul<f64>`: Multiplies an instance of the struct by a scalar of type `f64`.
/// - `std::ops::Div<f64>`: Divides an instance of the struct by a scalar of type `f64`.
/// - `std::cmp::PartialEq`: Compares two instances of the struct for equality.
/// 
/// # Example
/// ```rust
/// struct MyStruct(f64);
/// 
/// create_operations!(MyStruct);
/// 
/// let a = MyStruct(1.0);
/// let b = MyStruct(2.0);
/// let c = a + b; // MyStruct(3.0)
/// let d = c - a; // MyStruct(2.0)
/// let e = d * 2.0; // MyStruct(4.0)
/// let f = e / 2.0; // MyStruct(2.0)
/// assert_eq!(d, f); // true
/// ```
macro_rules! create_operations {
    ($struct_name:ident) => {
        impl std::ops::Add for $struct_name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl std::ops::Sub for $struct_name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl std::ops::Mul<f64> for $struct_name {
            type Output = Self;

            fn mul(self, scalar: f64) -> Self {
                Self(self.0 * scalar)
            }
        }

        impl std::ops::Div<f64> for $struct_name {
            type Output = Self;

            fn div(self, scalar: f64) -> Self {
                Self(self.0 / scalar)
            }
        }

        impl std::cmp::PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    };
}

#[macro_export]

macro_rules! create_dimension {
    ($struct_name:ident) => {
        impl crate::dimension::Dimension for $struct_name {
            fn as_base_units(&self) -> f64 {
                self.0
            }

            fn from_base_units(value: f64) -> Self {
                $struct_name(value)
            }
        }
    };
}

#[macro_export]
/// This macro generates multiplication and division operations between two different unit structures, resulting in a third unit structure.
/// 
/// # Parameters
/// - `$lhs_struct`: The left-hand side unit structure for the operation.
/// - `$rhs_struct`: The right-hand side unit structure for the operation.
/// - `$result_struct`: The resulting unit structure from the operation.
/// 
/// # Generated Implementations
///     
/// - `std::ops::Mul<$rhs_struct>`: Multiplies an instance of the left-hand side struct by an instance of the right-hand side struct, resulting in an instance of the result struct.
/// - `std::ops::Div<$rhs_struct>`: Divides an instance of the left-hand side struct by an instance of the right-hand side struct, resulting in an instance of the result struct.
/// 
/// 
/// # Example
/// ```rust
/// struct Length(f64);
/// struct Time(f64);
/// struct Speed(f64);
/// 
/// create_unit_operations!(Length / Time => Speed);
/// 
/// let length = Length(100.0);
/// let time = Time(10.0);
/// let speed = length / time; // Speed(10.0)
/// 
/// ```
/// 
/// # Note
/// This macro is intended to be used in conjunction with the `create_unit` macro to define unit operations between different unit structures.
macro_rules! create_unit_operations {
    ($lhs_struct:ident * $rhs_struct:ident => $result_struct:ident) => {
        impl std::ops::Mul<$rhs_struct> for $lhs_struct {
            type Output = $result_struct;

            fn mul(self, rhs: $rhs_struct) -> $result_struct {
                $result_struct(self.0 * rhs.0)
            }
        }
    };

    ($lhs_struct:ident / $rhs_struct:ident => $result_struct:ident) => {
        impl std::ops::Div<$rhs_struct> for $lhs_struct {
            type Output = $result_struct;

            fn div(self, rhs: $rhs_struct) -> $result_struct {
                $result_struct(self.0 / rhs.0)
            }
        }
    };
}

#[macro_export]
/// This macro combines the functionality of `create_converters`, `create_operations`, and `create_dimension` to create a complete unit structure with all necessary methods and traits.
/// 
/// # Parameters
/// - `$struct_name`: The name of the struct representing the unit.
/// - `$( $unit_name:ident => $conversion_factor:expr ),+`: A list of unit names and their conversion factors to the base unit.
/// 
/// # Generated Implementations
/// 
/// - Conversion methods for each unit in the list.
/// - Arithmetic operations (`Add`, `Sub`, `Mul`, `Div`) for the unit.
/// - `Dimension` trait implementation for the unit.
/// 
/// # Example
/// ```rust
/// create_unit!(Length, meter => 1.0, kilometer => 1000.0);
/// ```
/// 
/// This will generate a `Length` struct with conversion methods, arithmetic operations, and the `Dimension` trait implementation.
/// 
/// # Note
/// This macro is a convenience method to create a complete unit structure with all necessary methods and traits.
/// It is recommended to use this macro instead of manually calling `create_converters`, `create_operations`, and `create_dimension`.
/// 
/// # See Also
/// - `create_converters`
/// - `create_operations`
/// - `create_dimension`
macro_rules! create_unit {
    ($struct_name:ident, $( $unit_name:ident => $conversion_factor:expr ),+ ) => {
        paste::paste!{
        #[derive(Debug, Clone, Copy)]
        pub struct $struct_name(pub f64);

        crate::create_converters!($struct_name, $( $unit_name => $conversion_factor ),+);
        crate::create_operations!($struct_name);
        crate::create_dimension!($struct_name);
        }
    };
}
