use super::common::{Quantity, UnitValue};
use derive_more::{Add, Div, Mul, Neg, Sub};
use std::fmt;

#[derive(PartialEq, PartialOrd, Add, Sub, Neg, Mul, Div, Debug)]
pub struct Current {
    value: f64,
}

pub enum CurrentUnit {
    Ampere,
    KiloAmpere,
    MilliAmpere,
}

impl UnitValue for CurrentUnit {
    fn value(&self) -> f64 {
        match self {
            CurrentUnit::Ampere => 1.0,
            CurrentUnit::KiloAmpere => 1_000.0,
            CurrentUnit::MilliAmpere => 0.001,
        }
    }
}

impl Quantity for Current {
    type Unit = CurrentUnit;
    fn default_unit() -> Self::Unit {
        CurrentUnit::Ampere
    }
    fn from_base_value(v: f64) -> Self {
        Current { value: v }
    }
    fn base_value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} A", self.to(CurrentUnit::Ampere))
    }
}
