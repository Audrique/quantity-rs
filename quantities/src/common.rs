pub trait UnitValue {
    fn value(&self) -> f64;
}

pub trait Quantity: Sized {
    type Unit: UnitValue;

    fn from_base_value(value: f64) -> Self;

    fn base_value(&self) -> f64;

    fn new(value: f64, unit: Self::Unit) -> Self {
        Self::from_base_value(value * unit.value())
    }

    fn to(&self, unit: Self::Unit) -> f64 {
        self.base_value() / unit.value()
    }
}