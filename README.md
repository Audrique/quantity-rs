# quantity-rs

Existing quantity libraries often aim to support as many quantities and units as possible. However, this often makes it complicated for clients to define their own quantities and/or units. `quantity-rs` solves this by making it easy and straightforward for users to define their own custom quantities.

## Example

```rust
use quantities::common::UnitValue;
use quantities_macros::quantity;

#[quantity(unit = PriceUnit, value_type = f32)]
pub struct Price;

pub enum PriceUnit {
    Euro,
    Dollar,
}

impl UnitValue for PriceUnit {
    fn value(&self) -> f32 {
        match self {
            PriceUnit::Euro => 1.0,
            PriceUnit::Dollar => 0.85,
        }
    }
}

```
