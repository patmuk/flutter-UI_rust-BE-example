default: gen lint

gen: mkdir_generated
  cd shell_flutter && flutter pub get
  cd shell_flutter && flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
  # cd shell_flutter && dart run build_runner build

lint:
  cd app_core && cargo fmt
  cd shell_flutter && dart format .

clean:
  cd shell_flutter && flutter clean
  cd app_core && cargo clean

mkdir_generated:
  mkdir -p app_core/src/bridge/frb_generated
  mkdir -p shell_flutter/lib/bridge/frb_generated

clean_gen:
  rm -rf app_core/src/bridge/frb_generated/*
  rm -rf shell_flutter/lib/bridge/frb_generated/*

clean_all: clean clean_gen

run:
  cd shell_flutter && flutter run

run_fresh: clean gen run

run_fresh_all: clean_all gen run
