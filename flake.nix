{
  description = "Rust development flake for cryptographic purposes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
           (rust-bin.selectLatestNightlyWith (toolchain:
              toolchain.default.override{
                extensions = [ "rust-src" "rust-analyzer" ];    
            }))
          ];
        };
    });
}
