{
  description = "Flutter toolchain. Installs all tools needed for flutter, with versions pinned for this project. Rust's own tooling handles the rust toolchain.";
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
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

  nixConfig = {
    substituters = [
      "https://cache.nixos.org"
      "https://cache.onyx.ovh"
    ];
    trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "cache.onyx.ovh:2wUG6wsx5slbKUgkHT6GJuQ5k2StuUc8ysZQ2W+fbxA="
    ];
  };

  outputs = { nixpkgs, flake-utils, android-nixpkgs, rust-overlay, ... }:
    # outputs = { nixpkgs, flake-utils, android-nixpkgs, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
          overlays = [ (import rust-overlay) ];
          # overlays = [ ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        # rustToolchain = fenix.packages.${system}.fromToolchainFile {
        #   file = ./rust-toolchain.toml;
        #   sha256 = "sha256-vMlz0zHduoXtrlu0Kj1jEp71tYFXyymACW8L4jzrzNA=";
        # };
        frb_version = "latest";
        flutter_rust_bridge_codegen = import ./nix/flutter_rust_bridge_codegen.nix {
          inherit pkgs frb_version;
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
        pinnedJDK = pkgs.jdk17;
        appleInputs =
          # if builtins.elem system [ "aarch64-darwin" "x86_64-darwin" ] then 
          [
            pkgs.cocoapods
            # enable clang for `cargo build` (otherwise iconv is not found)
            # disable for `flutter run`
            # pkgs.clang

            # pkgs.apple-sdk_15
            # pkgs.darwin.xcode_16_2
            # pkgs.darwin.libiconv
            # pkgs.libiconv
            # pkgs.iconv
          ];
        #  else [ ];
      in
      {
        devShells.default = pkgs.mkShellNoCC
          {
            name = "flutter-rust-dev-shell";
            buildInputs = with pkgs; [
              just
              # rustc
              # cargo
              # libiconv
              # rustToolchain
              flutter_rust_bridge_codegen
              flutter
              pinnedJDK
              # androidCustomPackage
              sd
            ]
            # ++ lib.optionals stdenv.isDarwin [ libiconv iconv ]
            ++ appleInputs;
            JAVA_HOME = pinnedJDK;
            # ANDROID_SDK_ROOT = "${androidCustomPackage}/share/android-sdk";
            # Use this to create an android emulator
            # however, this is not needed, as VSCode's Flutter Plugin can create emulators as well
            # AVD_package = "system-images;android-34;aosp_atd;arm64-v8a";
            # local_toolchain_path = "$PWD/.toolchain";
            # local_SDK_path = "${local_toolchain_path}/android";
            # local_AVD_path = "${local_SDK_path}/AVD";
            # avdmanager create avd --name android-34-pixel_8 --package '${AVD_package}' --device "pixel_8"
            # GRADLE_USER_HOME = " /home/admin0101/.gradle ";
            # GRADLE_OPTS = " - Dorg.gradle.project.android.aapt2FromMavenOverride=${androidCustomPackage}/share/android-sdk/build-tools/34.0.0/aapt2";
            #
            # LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath pkgs}:${pkgs.stdenv.cc.cc.lib}/lib";
            # LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath pkgs}:${pkgs.libiconv}/lib";
            # LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${pkgs.libiconv}/lib/libiconv.dylib";
            # LD_LIBRARY_PATH = "${pkgs.libiconv}/lib/libiconv.dylib";
            shellHook = ''
              export PATH=$(echo $PATH | sd "${pkgs.xcbuild.xcrun}/bin" "")
              unset DEVELOPER_DIR
              #  uncomment to enable flutter-rust-bridge-codegen logging
              #  export RUST_BACKTRACE=1
              #  export RUST_LOG="debug" 
            '';
          };
        formatter = pkgs.alejandra;
      }
    );
}
