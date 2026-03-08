# Note that here we can do it without +nightly because 
# of how we install the nightly version in the flake.nix
fmt:
	cargo fmt
	dprint fmt
	rumdl fmt .
	nix fmt flake.nix

lint:
	cargo clippy
	rumdl check --fix .

test:
	cargo test
	cargo test -p quantities-macros
