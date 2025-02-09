{
  description = "Flutter toolchain. Installs all tools needed for flutter, with versions pinned for this project. Rust's own tooling handles the rust toolchain.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        devshell.follows = "devshell";
      };
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { nixpkgs, flake-utils, android-nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
            android_sdk = {
              accept_license = true;
            };
          };
        };
        overlays = [ (import rust-overlay) ];
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
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
        pinnedJDK = pkgs.jdk17;
        xcode_version = "16.2.0";
        # xcode_version = "15.4.0";
        frb_version = "latest";
        flutter_rust_bridge_codegen = import ./nix/flutter_rust_bridge_codegen.nix {
          inherit pkgs frb_version;
        };
        appleInputs =
          if builtins.elem system [ "aarch64-darwin" "x86_64-darwin" ] then [
            pkgs.cocoapods
            pkgs.xcodes
            pkgs.clang
            # pkgs.apple-sdk_15
            # pkgs.darwin.xcode_16_2
          ] else [ ];
      in
      {
        devShells.default = pkgs.mkShellNoCC
          {
            name = "flutter-rust-dev-shell";
            buildInputs = with pkgs; [
              just
              # rustToolchain
              flutter
              pinnedJDK
              # androidCustomPackage
              flutter_rust_bridge_codegen
            ]
            # libiconv has to be added on a mac, other machines have it
            ++ appleInputs;
            # ++ lib.optionals stdenv.isDarwin [ libiconv ];
            JAVA_HOME = pinnedJDK;
            # ANDROID_SDK_ROOT = "${androidCustomPackage}/share/android-sdk";

            # Use this to create an android emulator
            # however, this is not needed, as VSCode's Flutter Plugin can create emulators as well
            # AVD_package = "system-images;android-34;aosp_atd;arm64-v8a";
            # local_toolchain_path = "$PWD/.toolchain";
            # local_SDK_path = "${local_toolchain_path}/android";
            # local_AVD_path = "${local_SDK_path}/AVD";
            # avdmanager create avd --name android-34-pixel_8 --package '${AVD_package}' --device "pixel_8"
            shellHook = ''
              	      #  uncomment to enable flutter-rust-bridge-codegen logging
              	      #  export RUST_BACKTRACE=1
              	      #  export RUST_LOG="debug" 

                      # installs or checks for the right xcode version
                      echo "installing xcode ${xcode_version}"
                      xcodes install ${xcode_version} --experimental-unxip # --directory "$PWD/.xcode"
                      xcodes select ${xcode_version}
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
