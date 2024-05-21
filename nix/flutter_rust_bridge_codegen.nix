{ pkgs ? import <nixpkgs> { } }:
# doc: https://ryantm.github.io/nixpkgs/languages-frameworks/rust/
pkgs.rustPlatform.buildRustPackage {
  name = "flutter_rust_bridge_codegen";
  version = "v2.0.0-dev.33";
  src = pkgs.fetchFromGitHub {
    owner = "fzyzcjy";
    repo = "flutter_rust_bridge";
    rev = "e99f84a"; # hash of the commit
    sha256 = "sha256-+BkFKLEqC69egFcphxgmdR62luu2xrKWR8T23/yPh5A="; # optained SHA-256 with nix-prefetch-url --unpack https://github.com/fzyzcjy/flutter_rust_bridge/archive/v2.0.0-dev.31.tar.gz
  };
  cargoSha256 = "sha256-FuWGU5vzfptVKjftsVY/rLGbYAHGXZqpJoYwdPCuqRE="; # pkgs.lib.fakeSha256; # set it to 'lib.fakeSha256' and run nix (develop/build). replace the sha from the error message.

  # Specify the path to the Cargo.toml if it's not in the root of the repository
  cargoToml = "frb_codegen/Cargo.toml";

  buildInputs = with pkgs.darwin.apple_sdk; [
    frameworks.CoreServices
  ];


  checkFlags = [
    # skipping these tests, as they rely on a specific directory structure, which nix messes up. We can't patch the tests.
    "--skip=library::codegen::config::internal_config_parser::tests::test_parse_rust_output_faulty"
    "--skip=library::codegen::config::internal_config_parser::tests::test_parse_single_rust_input"
    "--skip=library::codegen::config::internal_config_parser::tests::test_parse_wildcard_rust_input"
    "--skip=library::codegen::generator::api_dart::tests::test_functions"
    "--skip=library::codegen::generator::api_dart::tests::test_simple"
    "--skip=library::codegen::parser::tests::test_error_non_opaque_mut"
    "--skip=library::codegen::parser::tests::test_generics"
    "--skip=library::codegen::parser::tests::test_methods"
    "--skip=library::codegen::parser::tests::test_multi_input_file"
    "--skip=library::codegen::parser::tests::test_non_qualified_names"
    "--skip=library::codegen::parser::tests::test_qualified_names"
    "--skip=library::codegen::parser::tests::test_simple"
    "--skip=library::codegen::parser::tests::test_unused_struct_enum"
    "--skip=library::codegen::parser::tests::test_use_type_in_another_file"
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_mode_config_file"
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_mode_from_naive_generate_command_args"
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_mode_config_file_faulty_file"
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_mode_from_files_auto_pubspec_yaml"
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_mode_from_files_auto_flutter_rust_bridge_yaml"
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_mode_from_files_auto_pubspec_yaml_faulty"
    "--skip=tests::test_execute_generate_on_frb_example_dart_minimal"
    "--skip=tests::test_execute_generate_on_frb_example_pure_dart"
    "--skip=library::utils::logs::configure_opinionated_logging"
  ];

  meta = {
    description = "Flutter/Dart <-> Rust binding generator, feature-rich, but seamless and simple.";
    homepage = "https://github.com/fzyzcjy/flutter_rust_bridge";
    license = pkgs.lib.licenses.mit;
    maintainers = [ ];
  };
}
