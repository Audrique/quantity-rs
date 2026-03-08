# Note that here we can do it without +nightly because 
# of how we install the nightly version in the flake.nix
fmt:
	cargo fmt
	dprint fmt
	nix fmt flake.nix

lint:
	cargo clippy

test:
	cargo test
	cargo test -p quantities-macros
