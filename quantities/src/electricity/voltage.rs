use crate::common::{Quantity, UnitValue};
use derive_more::{Add, Div, Mul, Neg, Sub};
use std::fmt;

#[derive(PartialEq, PartialOrd, Add, Sub, Neg, Mul, Div, Debug)]
pub struct Voltage {
    value: f64,
}

pub enum VoltageUnit {
    Volt,
    KiloVolt,
    MilliVolt,
}

impl UnitValue for VoltageUnit {
    fn value(&self) -> f64 {
        match self {
            VoltageUnit::Volt => 1.0,
            VoltageUnit::KiloVolt => 1_000.0,
            VoltageUnit::MilliVolt => 0.001,
        }
    }
}

impl Quantity for Voltage {
    type Unit = VoltageUnit;
    fn from_base_value(v: f64) -> Self {
        Voltage { value: v }
    }
    fn base_value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} V", self.to(VoltageUnit::Volt))
    }
}
