{
  description = "Flutter toolchain. Installs all tools needed for flutter, with versions pinned for this project. Rust's own tooling handles the rust toolchain.";
  # nix flutter doesn't work: https://github.com/NixOS/nixpkgs/issues/243448
  # thus using a local installation
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    android-nixpkgs.url = "github:tadfisher/android-nixpkgs";
  };

  outputs = { nixpkgs, flake-utils, android-nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
            android_sdk = {
              accept_license = true;
            };
          };
        };
        androidCustomPackage = android-nixpkgs.sdk.${system} (
          sdkPkgs: with sdkPkgs; [
            cmdline-tools-latest
            build-tools-30-0-3
            # build-tools-33-0-2
            build-tools-34-0-0
            ndk-23-1-7779620
            # ndk-26-2-11394342
            platform-tools
            emulator
            #patcher-v4
            # platforms-android-28
            platforms-android-33
            platforms-android-34
            system-images-android-34-aosp-atd-arm64-v8a #basic image, 40% faster
            system-images-android-34-google-apis-arm64-v8a #google branded
            system-images-android-34-google-apis-playstore-arm64-v8a #google branded with playstore installed
          ]
        );
        AVD_package = "system-images;android-34;aosp_atd;arm64-v8a";
        pinnedJDK = pkgs.jdk17;
        # xcode_version = "15.3.0";
        xcode_version = "15.4.0";
        frb_version = "latest";
        flutter_rust_bridge_codegen = import ./nix/flutter_rust_bridge_codegen.nix {
          inherit pkgs frb_version;
        };
        local_toolchain_path = "$PWD/.toolchain";
        local_flutter_path = "${local_toolchain_path}/flutter-local";
        local_SDK_path = "${local_toolchain_path}/android";
        local_AVD_path = "${local_SDK_path}/AVD";
        flutter_version = "latest";
        flutter-local = import ./nix/flutter-local.nix {
          inherit pkgs local_flutter_path flutter_version;
        };
      in
      {
        devShells. default = pkgs.mkShellNoCC
          {
            name = "My-flutter-dev-shell";
            buildInputs = with pkgs; [
              just
              cocoapods
              ruby
              xcodes
              pinnedJDK
              androidCustomPackage
              flutter_rust_bridge_codegen
            ];
            JAVA_HOME = pinnedJDK;
            ANDROID_SDK_ROOT = "${androidCustomPackage}/share/android-sdk";
            shellHook = ''
              export ANDROID_SDK_HOME="${local_SDK_path}";
              export ANDROID_EMULATOR_HOME="${local_SDK_path}";
              export ANDROID_AVD_HOME="${local_AVD_path}";

              mkdir -p ${local_toolchain_path}
              # installs flutter locally, if not there already
              ${flutter-local.unpack_flutter}/bin/unpack_flutter
              export PATH="${local_flutter_path}/flutter/bin:$PATH"
              echo
              # installs or checks for the right xcode version
              echo "installing xcode ${xcode_version}"
              xcodes install ${xcode_version} --experimental-unxip # --directory "$PWD/.xcode"
              xcodes select ${xcode_version}
              echo
              echo "setup for android emulator" 
              mkdir -p ${local_AVD_path}
              avdmanager create avd --name android-34-pixel_8 --package '${AVD_package}' --device "pixel_8"
              echo
              #  GRADLE_USER_HOME=$HOME/gradle-user-home
              #  GRADLE_HOME=$HOME/gradle-home
            '';

            # GRADLE_USER_HOME = " /home/admin0101/.gradle ";
            # GRADLE_OPTS = " - Dorg.gradle.project.android.aapt2FromMavenOverride=${androidCustomPackage}/share/android-sdk/build-tools/34.0.0/aapt2";
          };
      }
    );
}

