#[cfg(feature = "electricity")]
use quantities::electricity::power::{Power, PowerUnit};

#[cfg(feature = "electricity")]
fn main() {
    let p1 = Power::new(1500.0, PowerUnit::Watt); // 1500 W
    let p2 = Power::new(2.0, PowerUnit::KiloWatt); // 2000 W

    println!("p1: {} ({} kW)", p1, p1.to_unit(PowerUnit::KiloWatt));
    println!("p2: {} ({} kW)", p2, p2.to_unit(PowerUnit::KiloWatt));

    let sum = p1 + p2;
    println!("sum: {} ({} kW)", sum, sum.to_unit(PowerUnit::KiloWatt));

    // Horsepower example
    let one_hp = Power::new(1.0, PowerUnit::Horsepower);
    println!("1 hp = {} W", one_hp.to_unit(PowerUnit::Watt));
}

#[cfg(not(feature = "electricity"))]
fn main() {
    eprintln!(
        "Example 'power' requires the 'electricity' feature. Run with:\n  cargo run --example power --features electricity"
    );
}
