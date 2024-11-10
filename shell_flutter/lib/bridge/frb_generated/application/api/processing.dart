// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../domain/todo_list.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'processing.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `process_with_lifecycle`, `process_with_lifecycle`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `source`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ProcessingError>>
abstract class ProcessingError implements RustOpaqueInterface {}

@freezed
sealed class Effect with _$Effect {
  const Effect._();

  const factory Effect.todoListEffectRenderTodoList(
    TodoListModelLock field0,
  ) = Effect_TodoListEffectRenderTodoList;
  const factory Effect.todoListEffectRenderTodoItem(
    TodoItem field0,
  ) = Effect_TodoListEffectRenderTodoItem;
}

@freezed
sealed class TodoCommand with _$TodoCommand {
  const TodoCommand._();

  const factory TodoCommand.addTodo(
    String field0,
  ) = TodoCommand_AddTodo;
  const factory TodoCommand.removeTodo(
    BigInt field0,
  ) = TodoCommand_RemoveTodo;
  const factory TodoCommand.cleanList() = TodoCommand_CleanList;

  Future<List<Effect>> process() =>
      RustLib.instance.api.crateApplicationApiProcessingTodoCommandProcess(
        that: this,
      );
}

@freezed
sealed class TodoQuery with _$TodoQuery {
  const TodoQuery._();

  const factory TodoQuery.allTodos() = TodoQuery_AllTodos;
  const factory TodoQuery.getTodo(
    BigInt field0,
  ) = TodoQuery_GetTodo;

  Future<List<Effect>> process() =>
      RustLib.instance.api.crateApplicationApiProcessingTodoQueryProcess(
        that: this,
      );
}
