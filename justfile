default: gen lint

gen:
    cd shell_flutter && flutter pub get
    cd shell_flutter && flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yml

lint:
    cd app_core && cargo fmt
    cd shell_flutter && dart format .

clean:
    cd shell_flutter && flutter clean
    cd app_core && cargo clean

clean_gen:
    rm -rf app_core/src/bridge/generated
    rm -rf shell_flutter/ios/Runner/generated
    rm -rf shell_flutter/macos/Runner/generated
    rm -rf shell_flutter/lib/bridge/generated

clean_all: clean clean_gen

serve *args='':
    cd shell_flutter && flutter pub run flutter_rust_bridge:serve {{args}}

build:
    cargo lipo
    cp target/universal/debug/libapp_core.a shell_flutter/ios/Runner/generated/.
    
run:
    just build
    cd shell_flutter && flutter run
# vim:expandtab:sw=4:ts=4
