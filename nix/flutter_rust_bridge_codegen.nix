{ pkgs ? import <nixpkgs> { }, frb_version }:
# doc: https://ryantm.github.io/nixpkgs/languages-frameworks/rust/
let
  latest_version = "v2.3.0";
  desired_version = if (frb_version == null || frb_version == "latest") then latest_version else frb_version;

  frb_source =
    if (desired_version == "v2.3.0") then
      pkgs.fetchFromGitHub
        {
          owner = "fzyzcjy";
          repo = "flutter_rust_bridge";
          rev = "c81a186cd7146b6be23799fac89b13eda616847e"; # hash of the commit
          hash = "sha256-KaibvTCqxMFWNYD2vapD//G9cgEF6XX93//RChjnlIg="; # optained SHA-256 with nix-prefetch-url --unpack https://github.com/fzyzcjy/flutter_rust_bridge/archive/v2.2.0.tar.gz
	  fetchSubmodules = true;
	}
    else if (desired_version == "v2.2.0") then
      pkgs.fetchFromGitHub
        {
          owner = "fzyzcjy";
          repo = "flutter_rust_bridge";
          rev = "42f8102622fd6abd1945f2f037e876ee7bc0daae"; # hash of the commit
          sha256 = "17v8nn3lidniiwc5xl6sds07bmkhszrq86fnf1zpw35x4w7zamr3"; # optained SHA-256 with nix-prefetch-url --unpack https://github.com/fzyzcjy/flutter_rust_bridge/archive/v2.2.0.tar.gz
        }
    else if (desired_version == "v2.1.0") then
      pkgs.fetchFromGitHub
        {
          owner = "fzyzcjy";
          repo = "flutter_rust_bridge";
          rev = "4a81440596e59bf3f771ba074bd2b5a3e355ff31"; # hash of the commit
          sha256 = "197rzyzlvh28r9xvdi9d9ns88zknhf8kswhwj3y2nkwsa8v5x9j0"; # optained SHA-256 with nix-prefetch-url --unpack https://github.com/fzyzcjy/flutter_rust_bridge/archive/v2.1.0.tar.gz
        } else
      "Unknown frb version: ${desired_version}";

  # pkgs.lib.fakeSha256; # set it to 'lib.fakeSha256' and run nix (develop/build). replace the sha from the error message.
  frb_cargoHash =
    if (desired_version == "v2.3.0") then
      "sha256-ySPRNw6ugh9P67NBx017YB+rJ5+VU8n9BGIx+8e3uM4="
    else if (desired_version == "v2.2.0") then
      "sha256-viHZjZ2cItQNbHxKyf9pPZ0jbNfdd3t0KKvqQxvwlSs="
    else if (desired_version == "v2.1.0") then
      "sha256-/6chfWkiYd8mas3xaK5AsIJJjsSTzW0vXUVSHrgi3HI="
    else
      "Unknown frb version: ${desired_version}";

in
pkgs.rustPlatform.buildRustPackage
{
  name = "flutter_rust_bridge_codegen";
  version = desired_version;
  src = frb_source;
  cargoHash = frb_cargoHash;

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
    "--skip=binary::commands_parser::tests::test_compute_codegen_config_from_both_file_and_command_line"
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
