{
  description = "markdown toc generator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
					inherit system;
				};
				rust_toolchain = fenix.packages.${system}.stable.toolchain;
				pkgsFor = nixpkgs.legacyPackages;
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
						rust_toolchain
					];
				};
				packages = {
					default = pkgsFor.${system}.callPackage ./. { };
				};
      });
}
