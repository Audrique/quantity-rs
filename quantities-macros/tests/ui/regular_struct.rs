use quantities_macros::quantity;
use quantities::common::UnitValue;

#[quantity(unit = Unit, value_type = f64)]
pub struct RegularStruct {
    value: f64
}

pub enum Unit {
    Base,
}

impl UnitValue for Unit {
    fn value(&self) -> f64 {
        match self {
           Unit::Base => 1.0,
        }
    }
}

fn main() {}