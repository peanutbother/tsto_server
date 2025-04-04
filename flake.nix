{
  description = "A flake for building tsto.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    rust-flake ={
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix={
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    # cargo-doc-live.url = "github:srid/cargo-doc-live";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.treefmt-nix.flakeModule
      #   inputs.process-compose-flake.flakeModule
      #   inputs.cargo-doc-live.flakeModule
      ];

      flake = {
        nix-health.default = {
          nix-version.min-required = "2.16.0";
          direnv.required = true;
        };
      };

      perSystem = { config, self', pkgs, lib, system, ... }: {
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

        packages.default = pkgs.stdenv.mkDerivation {
          inherit system;
          name = "tsto_server";
          src = pkgs.nix-gitignore.gitignoreSourcePure [] ./.;
          nativeBuildInputs = with pkgs; [
            cmake
            dioxus-cli
            makeWrapper
            nodePackages.npm
            pkg-config
            protobuf
            rustup
            wasm-bindgen-cli
          ];
          buildPhase = ''
            export RUSTUP_HOME=$(pwd)/.rustup
            export CARGO_HOME=$(pwd)/.rustup
            npm install
            dx bundle --fullstack --release --out-dir dist
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp -r dist/public $out/bin
            cp dist/server $out/bin/tsto_server
          '';
        };

        devShells.default = pkgs.mkShell {
          name = "tsto_server";
          inputsFrom = [
            config.treefmt.build.devShell
            self'.devShells.rust
          ];
          packages = with pkgs; [
            cargo-watch
            cmake
            dioxus-cli
            nodePackages.npm
            protobuf
            rustup
            sqlx-cli
            wasm-bindgen-cli
          ];
          shellHook = ''
            rustc --version
            dx --version
            sqlx --version
          '';
        };
      };
    };
}
