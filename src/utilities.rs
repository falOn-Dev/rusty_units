
#[macro_export]
macro_rules! create_converters {
    ($struct_name:ident, $( $unit_name:ident => $conversion_factor:expr ),+ ) => {
        paste::paste!{
        impl $struct_name {
            $(
                pub fn [< as_ $unit_name >](&self) -> f64 {
                    self.base_magnitude * $conversion_factor
                }

                pub fn [< from_ $unit_name >](value: f64) -> Self {
                    $struct_name {
                        base_magnitude: value / $conversion_factor,
                    }
                }
            )+
        }
        }
    };
}

#[macro_export]
macro_rules! create_operations {
    ($struct_name:ident) => {
        impl std::ops::Add for $struct_name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    base_magnitude: self.base_magnitude + other.base_magnitude,
                }
            }
        }

        impl std::ops::Sub for $struct_name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    base_magnitude: self.base_magnitude - other.base_magnitude,
                }
            }
        }

        impl std::ops::Mul<f64> for $struct_name {
            type Output = Self;

            fn mul(self, scalar: f64) -> Self {
                Self {
                    base_magnitude: self.base_magnitude * scalar,
                }
            }
        }

        impl std::ops::Div<f64> for $struct_name {
            type Output = Self;

            fn div(self, scalar: f64) -> Self {
                Self {
                    base_magnitude: self.base_magnitude / scalar,
                }
            }
        }

        impl std::cmp::PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.base_magnitude == other.base_magnitude
            }
        }
    };
}

#[macro_export]
macro_rules! create_dimension {
    ($struct_name:ident) => {
        impl crate::dimension::Dimension for $struct_name {
            fn as_base_units(&self) -> f64 {
                self.base_magnitude
            }

            fn from_base_units(value: f64) -> Self {
                $struct_name {
                    base_magnitude: value,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_unit_operations {
    ($lhs_struct:ident * $rhs_struct:ident => $result_struct:ident) => {
        impl std::ops::Mul<$rhs_struct> for $lhs_struct {
            type Output = $result_struct;

            fn mul(self, rhs: $rhs_struct) -> $result_struct {
                $result_struct {
                    base_magnitude: self.base_magnitude * rhs.base_magnitude,
                }
            }
        }
    };

    ($lhs_struct:ident / $rhs_struct:ident => $result_struct:ident) => {
        impl std::ops::Div<$rhs_struct> for $lhs_struct {
            type Output = $result_struct;

            fn div(self, rhs: $rhs_struct) -> $result_struct {
                $result_struct {
                    base_magnitude: self.base_magnitude / rhs.base_magnitude,
                }
            }
        }
    };
}