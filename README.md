# flutter-rust-bridge_crux_style

This is a setup of [rust in flutter (rinf)](https://rinf-docs.cunarist.com) following its [guide](https://cjycode.com/flutter_rust_bridge/tutorial_with_flutter.html) with small amendments to follow the idea of [crux](https://github.com/redbadger/crux).

Feel free to use this as a starting template. I hope this helps the curx project to implement a flutter target.

This branch migrated from [flutter-rust-bridge](https://github.com/fzyzcjy/flutter_rust_bridge), as this could not build anymore in version 1.18.4: The tenplate project used 1.14.0, and while the "with Flutter" example uses the version from root, both don't compile anymore.
Even though 'rinf' adds the overhead of protobuf serialization, it works at least. (A further reason is that "Rust-Flutter-Bridge" relives on cargo lipo, which is abandoned. Nevertheless, thanks to "Flutter-Rust-Bridge" (and while we are at it: "Rust in Flutter" and "Crux") for the great work and effort!).

## Project Structure
The [default structure](https://rinf-docs.cunarist.com/applying-template/) is as follows:
- '''./messages''' : Source for code generation in form of .proto files. Can contain subdirectories.
- '''./lib/messages''' : generated Dart code
- '''./native/hub/src/messages''' : generated Rust code
- '''./native/hub/''' is the entry point into rust, especially '''./native/hub/src/lib.rs'''. The name 'hub' cannot be changed! TODO try if the business logic can be in another crate outside of the flutter folder structure.
- '''./native/hub/src/bridge/''' should not be modified, as this is the generated and infrastructure code.
- '''./cargo.toml''' is the workspace file. This places the target folder at ''./.'' as well, so that the iOS/Android flutter build pipelines can find the compiled libs.


## How to
### compile
See the documentation [running and building](https://rinf-docs.cunarist.com/running-and-building/) as well as [how to write code](https://rinf-docs.cunarist.com/writing-code/#request-from-dart-response-from-rust).

Run as usually with
'''flutter run'''

When code changes:
'''rinf message'''
To generate new/changed interfaces from .proto files.  
There is a watch op0tion as well, so any chages in the .proto files eecute code generation automatically:
'''rinf message --watch'''
