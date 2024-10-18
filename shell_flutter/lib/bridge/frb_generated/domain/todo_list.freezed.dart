// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'todo_list.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$TodoCommand {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(BigInt field0) removeTodo,
    required TResult Function() cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(BigInt field0)? removeTodo,
    TResult? Function()? cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(BigInt field0)? removeTodo,
    TResult Function()? cleanList,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TodoCommand_AddTodo value) addTodo,
    required TResult Function(TodoCommand_RemoveTodo value) removeTodo,
    required TResult Function(TodoCommand_CleanList value) cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand_AddTodo value)? addTodo,
    TResult? Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult? Function(TodoCommand_CleanList value)? cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TodoCommand_AddTodo value)? addTodo,
    TResult Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult Function(TodoCommand_CleanList value)? cleanList,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TodoCommandCopyWith<$Res> {
  factory $TodoCommandCopyWith(
          TodoCommand value, $Res Function(TodoCommand) then) =
      _$TodoCommandCopyWithImpl<$Res, TodoCommand>;
}

/// @nodoc
class _$TodoCommandCopyWithImpl<$Res, $Val extends TodoCommand>
    implements $TodoCommandCopyWith<$Res> {
  _$TodoCommandCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$TodoCommand_AddTodoImplCopyWith<$Res> {
  factory _$$TodoCommand_AddTodoImplCopyWith(_$TodoCommand_AddTodoImpl value,
          $Res Function(_$TodoCommand_AddTodoImpl) then) =
      __$$TodoCommand_AddTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$TodoCommand_AddTodoImplCopyWithImpl<$Res>
    extends _$TodoCommandCopyWithImpl<$Res, _$TodoCommand_AddTodoImpl>
    implements _$$TodoCommand_AddTodoImplCopyWith<$Res> {
  __$$TodoCommand_AddTodoImplCopyWithImpl(_$TodoCommand_AddTodoImpl _value,
      $Res Function(_$TodoCommand_AddTodoImpl) _then)
      : super(_value, _then);

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$TodoCommand_AddTodoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$TodoCommand_AddTodoImpl extends TodoCommand_AddTodo {
  const _$TodoCommand_AddTodoImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'TodoCommand.addTodo(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TodoCommand_AddTodoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TodoCommand_AddTodoImplCopyWith<_$TodoCommand_AddTodoImpl> get copyWith =>
      __$$TodoCommand_AddTodoImplCopyWithImpl<_$TodoCommand_AddTodoImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(BigInt field0) removeTodo,
    required TResult Function() cleanList,
  }) {
    return addTodo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(BigInt field0)? removeTodo,
    TResult? Function()? cleanList,
  }) {
    return addTodo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(BigInt field0)? removeTodo,
    TResult Function()? cleanList,
    required TResult orElse(),
  }) {
    if (addTodo != null) {
      return addTodo(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TodoCommand_AddTodo value) addTodo,
    required TResult Function(TodoCommand_RemoveTodo value) removeTodo,
    required TResult Function(TodoCommand_CleanList value) cleanList,
  }) {
    return addTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand_AddTodo value)? addTodo,
    TResult? Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult? Function(TodoCommand_CleanList value)? cleanList,
  }) {
    return addTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TodoCommand_AddTodo value)? addTodo,
    TResult Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult Function(TodoCommand_CleanList value)? cleanList,
    required TResult orElse(),
  }) {
    if (addTodo != null) {
      return addTodo(this);
    }
    return orElse();
  }
}

abstract class TodoCommand_AddTodo extends TodoCommand {
  const factory TodoCommand_AddTodo(final String field0) =
      _$TodoCommand_AddTodoImpl;
  const TodoCommand_AddTodo._() : super._();

  String get field0;

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TodoCommand_AddTodoImplCopyWith<_$TodoCommand_AddTodoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$TodoCommand_RemoveTodoImplCopyWith<$Res> {
  factory _$$TodoCommand_RemoveTodoImplCopyWith(
          _$TodoCommand_RemoveTodoImpl value,
          $Res Function(_$TodoCommand_RemoveTodoImpl) then) =
      __$$TodoCommand_RemoveTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt field0});
}

/// @nodoc
class __$$TodoCommand_RemoveTodoImplCopyWithImpl<$Res>
    extends _$TodoCommandCopyWithImpl<$Res, _$TodoCommand_RemoveTodoImpl>
    implements _$$TodoCommand_RemoveTodoImplCopyWith<$Res> {
  __$$TodoCommand_RemoveTodoImplCopyWithImpl(
      _$TodoCommand_RemoveTodoImpl _value,
      $Res Function(_$TodoCommand_RemoveTodoImpl) _then)
      : super(_value, _then);

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$TodoCommand_RemoveTodoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class _$TodoCommand_RemoveTodoImpl extends TodoCommand_RemoveTodo {
  const _$TodoCommand_RemoveTodoImpl(this.field0) : super._();

  @override
  final BigInt field0;

  @override
  String toString() {
    return 'TodoCommand.removeTodo(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TodoCommand_RemoveTodoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TodoCommand_RemoveTodoImplCopyWith<_$TodoCommand_RemoveTodoImpl>
      get copyWith => __$$TodoCommand_RemoveTodoImplCopyWithImpl<
          _$TodoCommand_RemoveTodoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(BigInt field0) removeTodo,
    required TResult Function() cleanList,
  }) {
    return removeTodo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(BigInt field0)? removeTodo,
    TResult? Function()? cleanList,
  }) {
    return removeTodo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(BigInt field0)? removeTodo,
    TResult Function()? cleanList,
    required TResult orElse(),
  }) {
    if (removeTodo != null) {
      return removeTodo(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TodoCommand_AddTodo value) addTodo,
    required TResult Function(TodoCommand_RemoveTodo value) removeTodo,
    required TResult Function(TodoCommand_CleanList value) cleanList,
  }) {
    return removeTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand_AddTodo value)? addTodo,
    TResult? Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult? Function(TodoCommand_CleanList value)? cleanList,
  }) {
    return removeTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TodoCommand_AddTodo value)? addTodo,
    TResult Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult Function(TodoCommand_CleanList value)? cleanList,
    required TResult orElse(),
  }) {
    if (removeTodo != null) {
      return removeTodo(this);
    }
    return orElse();
  }
}

abstract class TodoCommand_RemoveTodo extends TodoCommand {
  const factory TodoCommand_RemoveTodo(final BigInt field0) =
      _$TodoCommand_RemoveTodoImpl;
  const TodoCommand_RemoveTodo._() : super._();

  BigInt get field0;

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TodoCommand_RemoveTodoImplCopyWith<_$TodoCommand_RemoveTodoImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$TodoCommand_CleanListImplCopyWith<$Res> {
  factory _$$TodoCommand_CleanListImplCopyWith(
          _$TodoCommand_CleanListImpl value,
          $Res Function(_$TodoCommand_CleanListImpl) then) =
      __$$TodoCommand_CleanListImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$TodoCommand_CleanListImplCopyWithImpl<$Res>
    extends _$TodoCommandCopyWithImpl<$Res, _$TodoCommand_CleanListImpl>
    implements _$$TodoCommand_CleanListImplCopyWith<$Res> {
  __$$TodoCommand_CleanListImplCopyWithImpl(_$TodoCommand_CleanListImpl _value,
      $Res Function(_$TodoCommand_CleanListImpl) _then)
      : super(_value, _then);

  /// Create a copy of TodoCommand
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$TodoCommand_CleanListImpl extends TodoCommand_CleanList {
  const _$TodoCommand_CleanListImpl() : super._();

  @override
  String toString() {
    return 'TodoCommand.cleanList()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TodoCommand_CleanListImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(BigInt field0) removeTodo,
    required TResult Function() cleanList,
  }) {
    return cleanList();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(BigInt field0)? removeTodo,
    TResult? Function()? cleanList,
  }) {
    return cleanList?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(BigInt field0)? removeTodo,
    TResult Function()? cleanList,
    required TResult orElse(),
  }) {
    if (cleanList != null) {
      return cleanList();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TodoCommand_AddTodo value) addTodo,
    required TResult Function(TodoCommand_RemoveTodo value) removeTodo,
    required TResult Function(TodoCommand_CleanList value) cleanList,
  }) {
    return cleanList(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand_AddTodo value)? addTodo,
    TResult? Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult? Function(TodoCommand_CleanList value)? cleanList,
  }) {
    return cleanList?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TodoCommand_AddTodo value)? addTodo,
    TResult Function(TodoCommand_RemoveTodo value)? removeTodo,
    TResult Function(TodoCommand_CleanList value)? cleanList,
    required TResult orElse(),
  }) {
    if (cleanList != null) {
      return cleanList(this);
    }
    return orElse();
  }
}

abstract class TodoCommand_CleanList extends TodoCommand {
  const factory TodoCommand_CleanList() = _$TodoCommand_CleanListImpl;
  const TodoCommand_CleanList._() : super._();
}
