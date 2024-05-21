# How to

In general: read the [manual](https://cjycode.com/flutter_rust_bridge/)

## about nix
The needed toolchain is installed via nix (flake). Activate with `direnv allow` or `nix develop`. 

### toolchain only in cli
Nix (flake) installs the toolchain, which includes the environment variables such (as ANDROID_SDK_ROOT) only in the cli.
TO [have these available](https://discourse.nixos.org/t/flutter-in-vscode-does-not-see-my-android-device-but-flutter-from-the-terminal-does/20754), start codium from a flake directory. 

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
just gen
'''

If you want to start a new flutter project, do that with
'''
flutter_rust_bridge_codegen create --rust-crate-name app_core --rust-crate-dir ../app_core shell_flutter
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

'log stream --predicate="subsystem contains 'com.example.todo_app'" --level debug'
will show the log output in real-time.
'--level' is needed to set the log level, the default is "info". Note that "debug" is not Rust's logging system's debug level, but outputs "trace" log messages as well.
The '--predicate' "subsystem contains 'com.example.todo_app'" filters for whichever string you provided in the constructor of the logger (`OsLogger::new("com.example.todo_app")`).

One could filter for the process, like:
`log stream --process=app_core --level debug`
but this depends on how the app was started:
"app_core" if started from 'app_core/src/main.rs' or "shell_cli" if started from 'shell_cli/src/main.rs'.

One can filter for a spcific log message using the predicate:
'''
log stream --predicate 'eventMessage contains "Persisted"'
'''
Filters all log messages for the ones containing the word "Persisted".

#### Console.app

In the Console.app one needs to apply similar filters.
On the left pannel select your device.
In the search filed in the upper-left corner one can enter
'''
subsystem:com.example.todo_app
'''
Or
'''
process:app_core
'''
to filter for the executed app.

The log level can be changed by the menu `Action -> Include Debug Messages`

### see the logs in the iPhone Simulator

You can use either the command line or Console.app:

#### Command Line

Use the exact same command as above. If not filtering for the subsystem but the process the name is `Runner`.

#### Console.app

Exactly like above, but select the Simulator in "Devices" on the left pannel.
