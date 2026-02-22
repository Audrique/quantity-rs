pub trait UnitValue<T = f64> {
    fn value(&self) -> T;
}

pub trait Quantity<T = f64>: Sized + Clone {
    type Unit: UnitValue<T>;

    fn raw(value: T) -> Self;
    fn raw_value(&self) -> T;

    fn new(value: T, unit: Self::Unit) -> Self
    where
        T: std::ops::Mul<Output = T>,
    {
        Self::raw(value * unit.value())
    }
    fn to(&self, unit: Self::Unit) -> T
    where
        T: std::ops::Div<Output = T>,
    {
        self.raw_value() / unit.value()
    }
}
