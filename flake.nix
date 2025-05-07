{
  description = "Flutter-Rust-Bridge Toolchain. Tested on Apple Silicon.";
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
      local_toolchain_path = "$PWD/.toolchain";
      local_flutter_path = "${local_toolchain_path}/flutter-local";
      flutter_version = "latest";
      flutter-local = import
        ./nix/flutter-local.nix
        {
          inherit pkgs local_flutter_path flutter_version;
        };
      frb_version = "latest";
      flutter_rust_bridge_codegen = import ./nix/flutter_rust_bridge_codegen.nix {
        inherit pkgs frb_version;
      };
      rustToolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
      };
      androidCustomPackage = android-nixpkgs.sdk.${system} (
        # show all potential values with
        # nix flake show github:tadfisher/android-nixpkgs
        sdkPkgs: with sdkPkgs; [
          cmdline-tools-latest
          # cmdline-tools-17-0 
          build-tools-33-0-1
          build-tools-34-0-0
          build-tools-35-0-0
          ndk-23-1-7779620
          ndk-26-1-10909125
          ndk-28-0-13004108
          platform-tools
          emulator
          platforms-android-33
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
      pinnedJDK = pkgs.jdk17_headless;
    in
    {
      devShells.default = pkgs.mkShellNoCC
        {
          strictDeps = true;
          packages = with pkgs; [
            (xcodeenv.composeXcodeWrapper { versions = [ "16.2" ]; })
            cocoapods
            # broken: https://github.com/flutter/flutter/issues/167823
            # flutter
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
          # local_SDK_path = "${local_toolchain_path}/android";
          # local_AVD_path = "${local_SDK_path}/AVD";
          # avdmanager create avd --name android-34-pixel_8 --package '${AVD_package}' --device "pixel_8"
          JAVA_HOME = pinnedJDK;
          shellHook = ''
            unset DEVELOPER_DIR
            unset SDKROOT

            mkdir -p ${local_toolchain_path}
            # installs flutter locally, if not there already
            ${flutter-local.unpack_flutter}/bin/unpack_flutter
            export PATH="${local_flutter_path}/flutter/bin:$PATH"
            echo
          '';
          # FLUTTER_ROOT = "${pkgs.flutter}";
        };
    }
    );
}
