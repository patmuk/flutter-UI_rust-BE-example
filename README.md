# flutter-rust-bridge_crux_style

This is a setup of [rust in flutter (rinf)](https://rinf-docs.cunarist.com) following its [guide](https://cjycode.com/flutter_rust_bridge/tutorial_with_flutter.html) with small amendments to follow the idea of [crux](https://github.com/redbadger/crux).

Feel free to use this as a starting template. I hope this helps the curx project to implement a flutter target.

This branch migrated from [flutter-rust-bridge](https://github.com/fzyzcjy/flutter_rust_bridge), as this could not build anymore in version 1.18.4: The tenplate project used 1.14.0, and while the "with Flutter" example uses the version from root, both don't compile anymore.
Even though 'rinf' adds the overhead of protobuf serialization, it works at least. 
