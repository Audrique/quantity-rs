use crate::common::UnitValue;
use quantities_macros::quantity;

#[quantity(unit = VoltageUnit)]
pub struct Voltage;

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
