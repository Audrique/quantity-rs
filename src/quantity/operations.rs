use std::ops::{Div, Mul};

use super::common::Quantity;
use super::{current::Current, power::Power, voltage::Voltage};

impl Mul<Current> for Voltage {
    type Output = Power;
    fn mul(self, rhs: Current) -> Power {
        Power::from_base_value(self.base_value() * rhs.base_value())
    }
}
impl Mul<Voltage> for Current {
    type Output = Power;
    fn mul(self, rhs: Voltage) -> Power {
        Power::from_base_value(self.base_value() * rhs.base_value())
    }
}

impl Div<Voltage> for Power {
    type Output = Current;
    fn div(self, rhs: Voltage) -> Current {
        Current::from_base_value(self.base_value() / rhs.base_value())
    }
}

impl Div<Current> for Power {
    type Output = Voltage;
    fn div(self, rhs: Current) -> Voltage {
        Voltage::from_base_value(self.base_value() / rhs.base_value())
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::quantity::{current::CurrentUnit, power::PowerUnit, voltage::VoltageUnit};

    fn voltage() -> Voltage {
        Voltage::new(230.0, VoltageUnit::Volt)
    }

    fn current() -> Current {
        Current::new(10.0, CurrentUnit::Ampere)
    }

    fn expected_power() -> Power {
        voltage() * current()
    }

    #[test]
    fn current_times_voltage_gives_power() {
        assert_eq!(expected_power(), Power::new(2300.0, PowerUnit::Watt));
    }
}
