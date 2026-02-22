use crate::common::UnitValue;
use quantities_macros::quantity;

#[quantity(unit = CurrentUnit, value_type = f64)]
pub struct Current;

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

impl std::fmt::Display for Current {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} A", self.to(CurrentUnit::Ampere))
    }
}
