{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
        overlays = [ ];
      };
      xcodeenv = import (nixpkgs + "/pkgs/development/mobile/xcodeenv") { inherit (pkgs) callPackage; };
      frb_version = "latest";
      flutter_rust_bridge_codegen = import ./nix/flutter_rust_bridge_codegen.nix {
        inherit pkgs frb_version;
      };
      rustToolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-AJ6LX/Q/Er9kS15bn9iflkUwcgYqRQxiOIL2ToVAXaU=";
      };
    in
    {
      devShells.default = pkgs.mkShellNoCC
        {
          strictDeps = true;
          packages = with pkgs; [
            (xcodeenv.composeXcodeWrapper { versions = [ "16.2" ]; })
            cocoapods
            flutter
            flutter_rust_bridge_codegen
            rustToolchain
            just
          ];
          shellHook = ''
            unset DEVELOPER_DIR
            unset SDKROOT
          '';
        };
    }
    );
}
