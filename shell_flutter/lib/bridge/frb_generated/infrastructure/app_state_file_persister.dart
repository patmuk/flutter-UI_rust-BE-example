// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../application/api/lifecycle.dart';
import '../domain/todo_category.dart';
import '../domain/todo_list.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AppStateFilePersisterError>>
abstract class AppStateFilePersisterError
    implements RustOpaqueInterface, AppStatePersistError {
  /// convert to ProcessingError::NotPersisted
  @override
  Future<ProcessingError> toProcessingError();
}