{
  description = "A rewriating of lnprototest library for black box testing of the lightning network protocol";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    goflake.url = "github:sagikazarmark/go-flake";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, goflake, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
       pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
      in {
        packages = {
          default = pkgs.gnumake;
        };
        formatter = pkgs.nixpkgs-fmt;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # build dependencies
            libcap
            gcc
            pkg-config
            git

            gnumake
            golangci-lint

            go
            rustup

            # integration test dependencies
            clightning
            bitcoind
          ];

          shellHook = ''
            export CGO_ENABLED=0
            export HOST_CC=gcc
            export PWD=$(pwd)

            make dep
          '';
        };
      }
    );
}
