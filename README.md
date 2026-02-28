# quantity-rs

A minimal library for defining and working with quantities.

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
