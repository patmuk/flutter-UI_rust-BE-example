{ pkgs ? import <nixpkgs> { }, local_flutter_path, flutter_version }:
# Downloads flutter into a local directory (passed as local_flutter_path) and returns the bin path so the calling shellHook can set it to the PATH
# call (e.g. in ShellHook) with
#   ${flutter-local.unpack_flutter}/bin/unpack_flutter
#   export PATH="${local_flutter_path}/flutter/bin:$PATH"
# add a new flutter version:
# check the url in https://docs.flutter.dev/release/archive?tab=macos
# leave `hash = ""` and run `nix develop`. The error message will tell the correct hash value.
rec {
  latest_version = "3.24.1";
  desired_version = if (flutter_version == null || flutter_version == "latest") then latest_version else flutter_version;

  flutter_source =
    if desired_version == "3.24.1" then
      pkgs.fetchurl
        {
          url = "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_3.24.1-stable.zip";
          hash = "sha256-hHXmtkwPSEemKFR207P8tr2XKRXR6t71j/dpv62GFm8=";
        } else if desired_version == "3.24.0" then
      pkgs.fetchurl
        {
          url = "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_3.24.0-stable.zip";
          hash = "sha256-lKtzuIpKmWxOuTBkSDjQhjkYy8SgOagUTMr+Lhys0wQ=";
        } else if desired_version == "3.22.2" then
      pkgs.fetchurl
        {
          url = "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_3.22.2-stable.zip";
          hash = "sha256-1pgHHdxArPbrc6aHMwa5hwJDLUFRmun97PF27w3IbOM=";
        } else if desired_version == "3.19.6" then
      pkgs.fetchurl
        {
          url = "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_arm64_3.19.6-stable.zip";
          hash = "sha256-TmEUDfSWUr+PweMXUqb6hRj6TwW8bkxrywHI3/bZv48=";
        } else if desired_version == "3.19.5" then
      pkgs.fetchurl
        {
          url = "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_arm64_3.19.5-stable.zip";
          hash = "sha256-HXHsHs2bzt9Xaqp6cUyK/S/Qk028jqCfSx3DF31HX/Q=";
        } else if desired_version == "3.19.4" then
      pkgs.fetchurl
        {
          url = "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_arm64_3.19.4-stable.zip";
          hash = "sha256-3WRnvepBcH2fQ70+ZeI+jVEZYJLF1JJQehe2Pd9ew/U=";
        } else
      "Unknown flutter version: ${desired_version}";

  unpack_flutter = pkgs.writeShellApplication {
    name = "unpack_flutter";
    runtimeInputs = with pkgs; [
      unzip
    ];

    text = ''
      flutter_bin_dir="${local_flutter_path}"/flutter/bin
      flutter_bin_file="$flutter_bin_dir"/flutter
 
      echo "flutter needs local installation? ..."
      if [ -f "$flutter_bin_file" ]; then
        local_flutter_version=$( $flutter_bin_file --version | grep -oP 'Flutter \K.*(?= • channel)')
        if [ "$local_flutter_version" = "${desired_version}" ]; then
          echo "flutter $local_flutter_version is already installed locally in '${local_flutter_path}'"
          install=false
        else
          echo "flutter is already installed locally, but the installed version '$local_flutter_version' is not the same as requested version '${desired_version}'.  Uninstalling..."
          rm -rf "${local_flutter_path}"
          install=true
        fi
      else
        install=true
      fi
      if [ "$install" = "true" ]; then
        echo "... installing flutter version '${desired_version}' locally in '${local_flutter_path}'"
        unzip "${flutter_source}" -d "${local_flutter_path}"
        echo "installed flutter version '${desired_version}' to '${local_flutter_path}'"
      fi
    '';
  };
}
