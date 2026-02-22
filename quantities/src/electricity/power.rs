use crate::common::{Quantity, UnitValue};
use derive_more::{Add, Div, Mul, Neg, Sub};
use std::fmt;
pub enum PowerUnit {
    Watt,
    KiloWatt,
    MilliWatt,
    MegaWatt,
    Horsepower,
}

impl UnitValue for PowerUnit {
    fn value(&self) -> f64 {
        match self {
            PowerUnit::Watt => 1.0,
            PowerUnit::KiloWatt => 1_000.0,
            PowerUnit::MilliWatt => 0.001,
            PowerUnit::MegaWatt => 1_000_000.0,
            PowerUnit::Horsepower => 745.699872,
        }
    }
}
#[derive(PartialEq, PartialOrd, Add, Sub, Neg, Mul, Div, Debug)]
pub struct Power {
    value: f64,
}

impl Quantity for Power {
    type Unit = PowerUnit;

    fn from_base_value(value: f64) -> Self {
        Power { value }
    }

    fn base_value(&self) -> f64 {
        self.value
    }
}

impl Power {
    pub fn new(value: f64, unit: PowerUnit) -> Self {
        <Self as Quantity>::new(value, unit)
    }

    pub fn to_unit(&self, unit: PowerUnit) -> f64 {
        self.to(unit)
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} W", self.to(PowerUnit::Watt))
    }
}
