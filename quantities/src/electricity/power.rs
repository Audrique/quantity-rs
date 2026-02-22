use crate::common::UnitValue;
use quantities_macros::quantity;
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
#[quantity(unit = PowerUnit)]
pub struct Power;

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} W", self.to(PowerUnit::Watt))
    }
}
