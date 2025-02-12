{
  description = "Flutter toolchain. Installs all tools needed for flutter, with versions pinned for this project. Rust's own tooling handles the rust toolchain.";
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devshell = {
      url = "github:numtide/devshell";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }:
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
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-vMlz0zHduoXtrlu0Kj1jEp71tYFXyymACW8L4jzrzNA=";
        };
        pinnedJDK = pkgs.jdk17;
        appleInputs =
          # if builtins.elem system [ "aarch64-darwin" "x86_64-darwin" ] then 
          [
            pkgs.cocoapods
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
              rustc
              cargo
              rustToolchain
              flutter_rust_bridge_codegen
              flutter
              pinnedJDK
            ]
            # ++ lib.optionals stdenv.isDarwin [ libiconv ]
            ++ appleInputs;
            JAVA_HOME = pinnedJDK;
            shellHook = ''
              	      #  uncomment to enable flutter-rust-bridge-codegen logging
              	      #  export RUST_BACKTRACE=1
              	      #  export RUST_LOG="debug" 
            '';
          };
        formatter = pkgs.alejandra;
      }
    );
}
