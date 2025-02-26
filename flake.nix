{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        # devshell.follows = "devshell";
      };
    };
  };
  outputs =
    { self, nixpkgs, flake-utils, fenix, android-nixpkgs }:
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
      androidCustomPackage = android-nixpkgs.sdk.${system} (
        sdkPkgs: with sdkPkgs; [
          cmdline-tools-latest
          # cmdline-tools-17-0 
          build-tools-34-0-0
          build-tools-35-0-0
          ndk-28-0-13004108
          platform-tools
          emulator
          platforms-android-34
          platforms-android-35
          system-images-android-34-aosp-atd-arm64-v8a #basic image, 40% faster
          system-images-android-34-google-apis-arm64-v8a #google branded
          system-images-android-34-google-apis-playstore-arm64-v8a #google branded with playstore installed
          system-images-android-35-aosp-atd-arm64-v8a #basic image, 40% faster
          system-images-android-35-google-apis-arm64-v8a #google branded
          system-images-android-35-google-apis-playstore-arm64-v8a #google branded with playstore installed
        ]
      );
      pinnedJDK = pkgs.jdk17;
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
            androidCustomPackage
            pinnedJDK
            just
          ];
          ANDROID_SDK_ROOT = "${androidCustomPackage}/share/android-sdk";
          # Use this to create an android emulator
          # however, this is not needed, as VSCode's Flutter Plugin can create emulators as well
          # to anable the hardware keyboad and the android buttons, go to
          # ~/.android/avd/<emu-name>/config.ini
          # and set `hw.keyboard = yes` and `hw.mainKeys = yes`
          # AVD_package = "system-images;android-34;aosp_atd;arm64-v8a";
          # local_toolchain_path = "$PWD/.toolchain";
          # local_SDK_path = "${local_toolchain_path}/android";
          # local_AVD_path = "${local_SDK_path}/AVD";
          # avdmanager create avd --name android-34-pixel_8 --package '${AVD_package}' --device "pixel_8"
          JAVA_HOME = pinnedJDK;
          shellHook = ''
            unset DEVELOPER_DIR
            unset SDKROOT
          '';
        };
    }
    );
}
