/// A trait representing a physical dimension that can be converted to and from base units.
/// 
/// This trait provides methods for converting a value to its base units representation
/// and for creating an instance of the dimension from a value in base units.
/// 
/// # Examples
/// 
/// ```
/// use rusty_units::dimension::Dimension;
/// 
/// struct Length(f64);
/// 
/// impl Dimension for Length {
///     fn as_base_units(&self) -> f64 {
///         self.0
///     }
/// 
///     fn from_base_units(value: f64) -> Self {
///         Length(value)
///     }
/// }
/// 
/// let length = Length(100.0);
/// assert_eq!(length.as_base_units(), 100.0);
/// let new_length = Length::from_base_units(200.0);
/// assert_eq!(new_length.0, 200.0);
/// ```
pub trait Dimension {
    fn as_base_units(&self) -> f64;
    fn from_base_units(value: f64) -> Self;
}