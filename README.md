# How to
In general: read the [manual](https://cjycode.com/flutter_rust_bridge/)

## install
Follow the [Quickstart](https://cjycode.com/flutter_rust_bridge/quickstart.html)
run
'''
cargo install flutter_rust_bridge_codegen
# or with cargo-binstall
cargo binstall flutter_rust_bridge_codegen
'''
### targets
#### Android
'''
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
'''
#### iOS
'''
# 64 bit targets (real device & simulator):
rustup target add aarch64-apple-ios x86_64-apple-ios
# New simulator target for Xcode 12 and later
rustup target add aarch64-apple-ios-sim
# 32 bit targets (you probably don't need these):
rustup target add armv7-apple-ios i386-apple-ios
'''
## generate code:
run the command
'''
flutter_rust_bridge_codegen app_core/.flutter_rust_bridge.yml
'''
## build
run
'''
flutter run
'''
in the flutter directory. This compiles the rust lib and binds it as well.