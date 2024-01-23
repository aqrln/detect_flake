{
  description = "Detect flaky tests and non-deterministic behaviour";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;

        rustDev = rust.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

      in
      {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo-dist
            cargo-release
            rustDev
          ];
        };

        packages.default = craneLib.buildPackage rec {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = [ ];
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src buildInputs;
          };
        };
      }
    );
}

