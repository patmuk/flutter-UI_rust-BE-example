// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../application/api/lifecycle.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'todo_list.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `command_add_todo`, `command_clean_list`, `command_remove_todo`, `query_get_all_todos`, `query_get_todo`
// These types are ignored because they are not used by any `pub` functions: `TodoListEffect`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `clone`, `deserialize`, `eq`, `eq`, `eq`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `serialize`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<TodoListModel>>
abstract class TodoListModel implements RustOpaqueInterface, CqrsModel {
  static Future<TodoListModel> default_() =>
      RustLib.instance.api.crateDomainTodoListTodoListModelDefault();

  /// This is how to access the fields of a heavy object behind a RustAutoOpaque.
  /// This is copying parts the content, which Dart needs to show to the user.
  ///
  /// If `items` would be `pub` FRB would automatically create a getter. However, this
  /// getter would `clone()` the `items` as well. As we pretend that a single item
  /// is heavy to clone, we use a custom function to `clone()` only the lightweight and
  /// only needed part for presentation.
  Future<List<String>> getTodosAsString();
}

class TodoItem {
  final String text;

  const TodoItem({
    required this.text,
  });

  @override
  int get hashCode => text.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TodoItem &&
          runtimeType == other.runtimeType &&
          text == other.text;
}

class TodoListModelLock {
  final TodoListModel lock;

  const TodoListModelLock({
    required this.lock,
  });

  static Future<TodoListModelLock> default_() =>
      RustLib.instance.api.crateDomainTodoListTodoListModelLockDefault();

  static Future<TodoListModelLock> forModel({required TodoListModel model}) =>
      RustLib.instance.api
          .crateDomainTodoListTodoListModelLockForModel(model: model);

  @override
  int get hashCode => lock.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TodoListModelLock &&
          runtimeType == other.runtimeType &&
          lock == other.lock;
}

@freezed
sealed class TodoListProcessingError with _$TodoListProcessingError {
  const TodoListProcessingError._();

  const factory TodoListProcessingError.todoDoesNotExist(
    BigInt field0,
  ) = TodoListProcessingError_TodoDoesNotExist;
}
