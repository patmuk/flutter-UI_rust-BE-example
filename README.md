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

## Log
To properly log across devices, the os_log crate is used.
The configuration in app_core is enough, so logs can be writte nfrom a shell (see shell_cli for an example).
### see the logs in MacOS
The log messages can be found via comand line or Console.app
#### Command line
'''
log stream --process=app_core --level debug
'''
will show the log output in real-time.
'--process' refers to the process (i.e. crate name) that was executed:
"app_core" if started from 'app_core/src/main.rs' or "shell_cli" if started from 'shell_cli/src/main.rs'.
'--level' is needed to set the log level, the default is "info". Note that "debug" is not Rust's logging system's debug level, but outputs "trace" log messages as well.
One can filter for a spcific log message with a predicate:
'''
log stream --predicate 'eventMessage contains "Persisted"'
'''
Filters all log messages for the ones containing the word "Persisted".
#### Console.app
In the Console.app one needs to apply similar filters.
On the left pannel select your device.
In the search filed in the upper-left corner one can enter 
'''
process:app_core
'''
to filter for the executed app.

### see the logs in the iPhone Simulator
