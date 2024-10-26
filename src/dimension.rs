pub trait Dimension {
    fn as_base_units(&self) -> f64;
    fn from_base_units(value: f64) -> Self;
}