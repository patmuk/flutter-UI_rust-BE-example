// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../application/api/lifecycle.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'todo_list.dart';
part 'todo_category.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `get_todo_category_model`, `get_todo_category`, `get_todo_list`, `set_todo_list`, `update_title`
// These types are ignored because they are neither used by any `pub` functions nor (for structs and enums) marked `#[frb(unignore)]`: `TodoCategoryEffect`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `deserialize`, `eq`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `serialize`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<TodoCategoryModel>>
abstract class TodoCategoryModel implements RustOpaqueInterface, CqrsModel {
  static Future<TodoCategoryModel> default_() =>
      RustLib.instance.api.crateDomainTodoCategoryTodoCategoryModelDefault();

  /// This is how to access the fields of a heavy object behind a RustAutoOpaque.
  /// This is copying parts the content, which Dart needs to show to the user.
  ///
  /// If `items` would be `pub` FRB would automatically create a getter. However, this
  /// getter would `clone()` the `items` as well. As we pretend that a single item
  /// is heavy to clone, we use a custom function to `clone()` only the lightweight and
  /// only needed part for presentation.
  Future<String> getTitle();

  Future<TodoListModelLock> getTodoListLock();

  Future<List<String>> getTodos();
}

class TodoCategoryModelLock {
  final TodoCategoryModel model;

  const TodoCategoryModelLock({
    required this.model,
  });

  static Future<TodoCategoryModelLock> forModel(
          {required TodoCategoryModel model}) =>
      RustLib.instance.api
          .crateDomainTodoCategoryTodoCategoryModelLockForModel(model: model);

  @override
  int get hashCode => model.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TodoCategoryModelLock &&
          runtimeType == other.runtimeType &&
          model == other.model;
}

@freezed
sealed class TodoCategoryProcessingError with _$TodoCategoryProcessingError {
  const TodoCategoryProcessingError._();

  const factory TodoCategoryProcessingError.todoDoesNotExist(
    BigInt field0,
  ) = TodoCategoryProcessingError_TodoDoesNotExist;
}
