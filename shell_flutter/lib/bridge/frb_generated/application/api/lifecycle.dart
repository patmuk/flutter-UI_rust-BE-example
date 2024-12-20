// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../domain/todo_category.dart';
import '../../domain/todo_list.dart';
import '../../frb_generated.dart';
import '../../lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'lifecycle.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `process_with_lifecycle`, `process_with_lifecycle`, `process_with_lifecycle`, `process_with_lifecycle`
// These functions are ignored because they have generic arguments: `init`, `init`, `load_or_new`, `load_or_new`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `source`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AppConfigImpl>>
abstract class AppConfigImpl implements RustOpaqueInterface, AppConfig {
  PathBuf get appStateFilePath;

  set appStateFilePath(PathBuf appStateFilePath);

  static Future<AppConfigImpl> default_() =>
      RustLib.instance.api.crateApplicationApiLifecycleAppConfigImplDefault();

  @override
  Future<void> getAppStateFilePath();

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  /// call to overwrite default values.
  /// Doesn't trigger initialization.
  static Future<AppConfigImpl> newInstance({String? path}) =>
      RustLib.instance.api
          .crateApplicationApiLifecycleAppConfigImplNew(path: path);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<LifecycleImpl>>
abstract class LifecycleImpl implements RustOpaqueInterface, Lifecycle {
  @override
  Future<void> appConfig();

  @override
  Future<void> appState();

  static Future<void> getSingleton() => RustLib.instance.api
      .crateApplicationApiLifecycleLifecycleImplGetSingleton();

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<void> newInstance({String? path}) => RustLib.instance.api
      .crateApplicationApiLifecycleLifecycleImplNew(path: path);

  /// persist the app state to the previously stored location
  @override
  Future<void> persist();

  @override
  Future<void> shutdown();
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ProcessingError>>
abstract class ProcessingError implements RustOpaqueInterface {}

abstract class AppConfig {
  Future<void> getAppStateFilePath();
}

abstract class AppState {
  Future<bool> dirtyFlagValue();

  Future<void> markDirty();

  Future<void> persistToPath({required PathBuf path});
}

abstract class Cqrs {
  Future<List<Effect>> process();
}

abstract class CqrsModel {}

abstract class CqrsModelLock {}

abstract class Lifecycle {
  Future<void> appConfig();

  Future<void> appState();

  Future<void> persist();

  Future<void> shutdown();
}

@freezed
sealed class Effect with _$Effect {
  const Effect._();

  /// this indicates that the model has changed, so that the app's state should be persisted.
  /// to avoid scanning the entire vec, this must be the first element.
  const factory Effect.todoListModelRenderTodoList(
    TodoListModelLock field0,
  ) = Effect_TodoListModelRenderTodoList;
  const factory Effect.todoListModelRenderTodoItem(
    TodoItem field0,
  ) = Effect_TodoListModelRenderTodoItem;
  const factory Effect.todoCategoryModelRenderTodoCategoryModel(
    TodoCategoryModelLock field0,
  ) = Effect_TodoCategoryModelRenderTodoCategoryModel;
  const factory Effect.todoCategoryModelRenderTodoCategory(
    String field0,
  ) = Effect_TodoCategoryModelRenderTodoCategory;
  const factory Effect.todoCategoryModelRenderTodoList(
    TodoListModelLock field0,
  ) = Effect_TodoCategoryModelRenderTodoList;
  const factory Effect.todoCategoryModelUpdateTitel(
    String field0,
  ) = Effect_TodoCategoryModelUpdateTitel;
}

@freezed
sealed class TodoCategoryModelCommand with _$TodoCategoryModelCommand {
  const TodoCategoryModelCommand._();

  const factory TodoCategoryModelCommand.setTodoList(
    TodoListModelLock field0,
  ) = TodoCategoryModelCommand_SetTodoList;
  const factory TodoCategoryModelCommand.updateTitle(
    String field0,
  ) = TodoCategoryModelCommand_UpdateTitle;

  Future<List<Effect>> process() => RustLib.instance.api
          .crateApplicationApiLifecycleTodoCategoryModelCommandProcess(
        that: this,
      );
}

enum TodoCategoryModelQuery {
  getTodoCategory,
  getTodoCategoryModel,
  getTodoList,
  ;

  Future<List<Effect>> process() => RustLib.instance.api
          .crateApplicationApiLifecycleTodoCategoryModelQueryProcess(
        that: this,
      );
}

@freezed
sealed class TodoListModelCommand with _$TodoListModelCommand {
  const TodoListModelCommand._();

  const factory TodoListModelCommand.addTodo(
    String field0,
  ) = TodoListModelCommand_AddTodo;
  const factory TodoListModelCommand.cleanList() =
      TodoListModelCommand_CleanList;
  const factory TodoListModelCommand.removeTodo(
    BigInt field0,
  ) = TodoListModelCommand_RemoveTodo;

  Future<List<Effect>> process() => RustLib.instance.api
          .crateApplicationApiLifecycleTodoListModelCommandProcess(
        that: this,
      );
}

@freezed
sealed class TodoListModelQuery with _$TodoListModelQuery {
  const TodoListModelQuery._();

  const factory TodoListModelQuery.getAllTodos() =
      TodoListModelQuery_GetAllTodos;
  const factory TodoListModelQuery.getTodo(
    BigInt field0,
  ) = TodoListModelQuery_GetTodo;

  Future<List<Effect>> process() => RustLib.instance.api
          .crateApplicationApiLifecycleTodoListModelQueryProcess(
        that: this,
      );
}
