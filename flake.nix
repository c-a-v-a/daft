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
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

        rsBuild = pkgs.rustPlatform.buildRustPackage rec {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;

          postPatch = ''cp ${./Cargo.lock} Cargo.lock'';
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
						rust_toolchain
					];
				};

        defaultPackage = rsBuild;

        packages = {
          default = rsBuild;
        };
      });
}
