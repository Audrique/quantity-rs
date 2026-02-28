use quantities_macros::quantity;
use quantities::common::UnitValue;

#[quantity(unit = RandomQuantityUnit, unit = RandomQuantityUnit)]
pub struct RandomQuantity;

pub enum RandomQuantityUnit{
    Base,
}

impl UnitValue for RandomQuantityUnit {
    fn value(&self) -> f64 {
        match self {
           RandomQuantityUnit::Base => 1.0,
        }
    }
}

fn main() {}