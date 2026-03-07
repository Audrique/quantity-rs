{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nightlyFmt = pkgs.rust-bin.nightly.latest.rustfmt;
      in
      {
        devShells.default = pkgs.mkShell {
          # Everything in this list gets added to your PATH
          packages = [
            # Nightly rustfmt, put it before rustToolchain otherwise 
            # the stable version will be used
            nightlyFmt         
            rustToolchain      # Provides cargo, rustc, rustfmt, clippy
            pkgs.just          # Command runner
            pkgs.dprint        # Formatter for toml, json, md and yaml
            pkgs.pkg-config    # Essential for building crates with C-deps
            pkgs.openssl       # Common dependency for web/crypto crates (e.g. needed for tokio)
          ];
        };
      });
}