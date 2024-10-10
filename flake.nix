{
  description = "Flutter toolchain. Installs all tools needed for flutter, with versions pinned for this project. Rust's own tooling handles the rust toolchain.";
  # nix flutter doesn't work: https://github.com/NixOS/nixpkgs/issues/243448
  # thus using a local installation
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
    # share rust configuration with nix ... not really needed!
    # rust-overlay = {
    #   url = "github:oxalica/rust-overlay";
    #   inputs = {
    #     nixpkgs.follows = "nixpkgs";
    #     flake-utils.follows = "flake-utils";
    #   };
    # };
    #
    # workaround (using the last working version)
    # until https://github.com/NixOS/nixpkgs/issues/327836 and https://github.com/NixOS/nixpkgs/issues/242779 are fixed
    # needs PR https://github.com/NixOS/nixpkgs/pull/346043 and https://github.com/NixOS/nixpkgs/pull/346947 to be merged
    # using last know version where swift wasn't broken
    swift-nixpkgs.url = "github:nixos/nixpkgs?rev=2e92235aa591abc613504fde2546d6f78b18c0cd";
  };

  # outputs = { nixpkgs, flake-utils, android-nixpkgs, rust-overlay, swift-nixpkgs, ... }:
  outputs = { nixpkgs, flake-utils, android-nixpkgs, swift-nixpkgs, ... }:
    # outputs = { nixpkgs, flake-utils, android-nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        swift-pkgs = import swift-nixpkgs {
          inherit system;
        };
        # overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          # inherit system overlays;
          inherit system;
          config = {
            allowUnfree = true;
            android_sdk = {
              accept_license = true;
            };
          };
        };
        # rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
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
        xcode_version = "15.4.0";
        frb_version = "latest";
        flutter_rust_bridge_codegen = import ./nix/flutter_rust_bridge_codegen.nix {
          inherit pkgs frb_version;
        };
        local_toolchain_path = "$PWD/.toolchain";
        local_SDK_path = "${local_toolchain_path}/android";
        local_AVD_path = "${local_SDK_path}/AVD";
      in
      {
        devShells. default = pkgs.mkShellNoCC
          {
            name = "flutter-rust-dev-shell";
            buildInputs = with pkgs; [
              just
              # rustToolchain
              flutter
              cocoapods
              swift-pkgs.xcodes
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
              	
                    	      #  uncomment to enable flutter-rust-bridge-codegen logging
                    	      #  export RUST_BACKTRACE=1
                    	      #  export RUST_LOG="debug" 

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

