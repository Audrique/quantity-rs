#[cfg(feature = "electricity")]
use quantities::electricity::power::{Power, PowerUnit};

#[cfg(feature = "electricity")]
fn main() {
	let p1 = Power::new(1500.0, PowerUnit::Watt); // 1500 W
	let p2 = Power::new(2.0, PowerUnit::KiloWatt); // 2000 W

	println!(
		"p1: {:?} ({:?} kW)",
		p1,
		p1.to(PowerUnit::KiloWatt)
	);
	println!(
		"p2: {:?} ({:?} kW)",
		p2,
		p2.to(PowerUnit::KiloWatt)
	);

	let diff = p1.clone() - p2.clone();
	println!(
		"diff: {:?} ({:?} W)",
		diff,
		diff.clone().to(PowerUnit::Watt)
	);

	let sum = p1 + p2;
	println!(
		"sum: {:?} ({:?} kW)",
		sum,
		sum.clone().to(PowerUnit::KiloWatt)
	);

	// Note that right multiplication does not work.
	// This is a choice as we want to keep flexibility
	// on the underlying type T and it is not possible
	// because of the orphan rule.
	println!(
		"The power multiplied by 2.0 from the right: {:?}",
		sum.clone() * 2.0
	);

	println!(
		"The power divided by 2.0: {:?}",
		sum.clone() / 2.0
	);

	// Horsepower example
	let one_hp = Power::new(1.0, PowerUnit::Horsepower);
	println!("1 hp = {:?} W", one_hp.to(PowerUnit::Watt));
}

#[cfg(not(feature = "electricity"))]
fn main() {
	eprintln!(
		"Example 'power' requires the 'electricity' feature. Run \
		 with:\n  cargo run --example power --features electricity"
	);
}
