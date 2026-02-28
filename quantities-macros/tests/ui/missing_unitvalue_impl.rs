use quantities_macros::quantity;

#[quantity(unit=RandomQuantityUnit)]
pub struct RandomQuantity;

enum RandomQuantityUnit {
    Base
}

fn main() {}