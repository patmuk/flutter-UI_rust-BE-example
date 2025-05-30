// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../domain/todo_category.dart';
import '../../domain/todo_list.dart';
import '../../frb_generated.dart';
import '../../infrastructure/app_state_file_persister.dart';
import '../app_config.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'lifecycle.freezed.dart';

// These functions are ignored because they have generic arguments: `initialise_with_app_config`, `initialise_with_app_config`, `load_app_state`, `load_app_state`, `load_app_state`, `new`, `new`, `new`, `new`, `new`, `persist_app_state`, `persist_app_state`, `persist_app_state`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<LifecycleImpl>>
abstract class LifecycleImpl implements RustOpaqueInterface, Lifecycle {
  AppConfigImpl get appConfig;

  set appConfig(AppConfigImpl appConfig);

  /// frb doesn't support generics. If needed implement them using enums or the enum_dispatch crate.
  static Future<void> getSingleton() => RustLib.instance.api
      .crateApplicationApiLifecycleLifecycleImplGetSingleton();

  /// frb doesn't support generics. If needed implement them using enums or the enum_dispatch crate.
  static Future<void> initialise({String? appStateUrl}) =>
      RustLib.instance.api.crateApplicationApiLifecycleLifecycleImplInitialise(
          appStateUrl: appStateUrl);

  /// persist the app state to the previously stored location
  /// frb doesn't support generics. If needed implement them using enums or the enum_dispatch crate.
  static Future<void> persist() =>
      RustLib.instance.api.crateApplicationApiLifecycleLifecycleImplPersist();

  /// frb doesn't support generics. If needed implement them using enums or the enum_dispatch crate.
  static Future<void> shutdown() =>
      RustLib.instance.api.crateApplicationApiLifecycleLifecycleImplShutdown();
}

abstract class AppConfig {
  /// app state storage location
  Future<void> borrowAppStateUrl();
}

abstract class AppState {
  Future<bool> dirtyFlagValue();

  Future<void> markDirty();

  Future<void> markPersisted();
}

abstract class AppStatePersistError {
  /// convert to ProcessingError::NotPersisted
  Future<ProcessingError> toProcessingError();
}

abstract class Cqrs {
  Future<List<Effect>> process();
}

abstract class CqrsModel {}

abstract class CqrsModelLock {}

abstract class Lifecycle {}

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
sealed class ProcessingError with _$ProcessingError {
  const ProcessingError._();

  const factory ProcessingError.todoListProcessingError(
    TodoListProcessingError field0,
  ) = ProcessingError_TodoListProcessingError;
  const factory ProcessingError.todoCategoryProcessingError(
    TodoCategoryProcessingError field0,
  ) = ProcessingError_TodoCategoryProcessingError;
  const factory ProcessingError.notPersisted({
    required String error,
    required String url,
  }) = ProcessingError_NotPersisted;
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

@freezed
sealed class TodoCategoryModelQuery with _$TodoCategoryModelQuery {
  const TodoCategoryModelQuery._();

  const factory TodoCategoryModelQuery.getTodoCategory(
    bool field0,
  ) = TodoCategoryModelQuery_GetTodoCategory;
  const factory TodoCategoryModelQuery.getTodoCategoryModel() =
      TodoCategoryModelQuery_GetTodoCategoryModel;
  const factory TodoCategoryModelQuery.getTodoList() =
      TodoCategoryModelQuery_GetTodoList;

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
