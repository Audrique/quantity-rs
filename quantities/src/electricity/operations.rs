use std::ops::{Div, Mul};

use crate::common::Quantity;
use crate::electricity::{Current, Power, Voltage};

impl<T> Mul<Current<T>> for Voltage<T>
where
	T: Mul<Output = T>,
	Voltage<T>: Quantity<T>,
	Current<T>: Quantity<T>,
	Power<T>: Quantity<T>,
{
	type Output = Power<T>;

	fn mul(self, rhs: Current<T>) -> Power<T> {
		Power::<T>::raw(self.raw_value() * rhs.raw_value())
	}
}

impl<T> Mul<Voltage<T>> for Current<T>
where
	T: Mul<Output = T>,
	Voltage<T>: Quantity<T>,
	Current<T>: Quantity<T>,
	Power<T>: Quantity<T>,
{
	type Output = Power<T>;

	fn mul(self, rhs: Voltage<T>) -> Power<T> {
		Power::<T>::raw(self.raw_value() * rhs.raw_value())
	}
}

impl<T> Div<Voltage<T>> for Power<T>
where
	T: Div<Output = T>,
	Voltage<T>: Quantity<T>,
	Current<T>: Quantity<T>,
	Power<T>: Quantity<T>,
{
	type Output = Current<T>;

	fn div(self, rhs: Voltage<T>) -> Current<T> {
		Current::<T>::raw(self.raw_value() / rhs.raw_value())
	}
}

impl<T> Div<Current<T>> for Power<T>
where
	T: Div<Output = T>,
	Voltage<T>: Quantity<T>,
	Current<T>: Quantity<T>,
	Power<T>: Quantity<T>,
{
	type Output = Voltage<T>;

	fn div(self, rhs: Current<T>) -> Voltage<T> {
		Voltage::<T>::raw(self.raw_value() / rhs.raw_value())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::electricity::current::CurrentUnit;
	use crate::electricity::power::PowerUnit;
	use crate::electricity::voltage::VoltageUnit;

	fn voltage() -> Voltage { Voltage::new(230.0, VoltageUnit::Volt) }

	fn current() -> Current {
		Current::new(10.0, CurrentUnit::Ampere)
	}

	#[test]
	fn current_times_voltage_gives_power() {
		assert_eq!(
			voltage() * current(),
			Power::new(2300.0, PowerUnit::Watt)
		);
	}
}
