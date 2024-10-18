// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lifecycle.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Cqrs {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(TodoCommand field0) todoCommand,
    required TResult Function(TodoQuery field0) todoQuery,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand field0)? todoCommand,
    TResult? Function(TodoQuery field0)? todoQuery,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(TodoCommand field0)? todoCommand,
    TResult Function(TodoQuery field0)? todoQuery,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommand value) todoCommand,
    required TResult Function(Cqrs_TodoQuery value) todoQuery,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommand value)? todoCommand,
    TResult? Function(Cqrs_TodoQuery value)? todoQuery,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommand value)? todoCommand,
    TResult Function(Cqrs_TodoQuery value)? todoQuery,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CqrsCopyWith<$Res> {
  factory $CqrsCopyWith(Cqrs value, $Res Function(Cqrs) then) =
      _$CqrsCopyWithImpl<$Res, Cqrs>;
}

/// @nodoc
class _$CqrsCopyWithImpl<$Res, $Val extends Cqrs>
    implements $CqrsCopyWith<$Res> {
  _$CqrsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$Cqrs_TodoCommandImplCopyWith<$Res> {
  factory _$$Cqrs_TodoCommandImplCopyWith(_$Cqrs_TodoCommandImpl value,
          $Res Function(_$Cqrs_TodoCommandImpl) then) =
      __$$Cqrs_TodoCommandImplCopyWithImpl<$Res>;
  @useResult
  $Res call({TodoCommand field0});

  $TodoCommandCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Cqrs_TodoCommandImplCopyWithImpl<$Res>
    extends _$CqrsCopyWithImpl<$Res, _$Cqrs_TodoCommandImpl>
    implements _$$Cqrs_TodoCommandImplCopyWith<$Res> {
  __$$Cqrs_TodoCommandImplCopyWithImpl(_$Cqrs_TodoCommandImpl _value,
      $Res Function(_$Cqrs_TodoCommandImpl) _then)
      : super(_value, _then);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Cqrs_TodoCommandImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as TodoCommand,
    ));
  }

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TodoCommandCopyWith<$Res> get field0 {
    return $TodoCommandCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Cqrs_TodoCommandImpl extends Cqrs_TodoCommand {
  const _$Cqrs_TodoCommandImpl(this.field0) : super._();

  @override
  final TodoCommand field0;

  @override
  String toString() {
    return 'Cqrs.todoCommand(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Cqrs_TodoCommandImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Cqrs_TodoCommandImplCopyWith<_$Cqrs_TodoCommandImpl> get copyWith =>
      __$$Cqrs_TodoCommandImplCopyWithImpl<_$Cqrs_TodoCommandImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(TodoCommand field0) todoCommand,
    required TResult Function(TodoQuery field0) todoQuery,
  }) {
    return todoCommand(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand field0)? todoCommand,
    TResult? Function(TodoQuery field0)? todoQuery,
  }) {
    return todoCommand?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(TodoCommand field0)? todoCommand,
    TResult Function(TodoQuery field0)? todoQuery,
    required TResult orElse(),
  }) {
    if (todoCommand != null) {
      return todoCommand(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommand value) todoCommand,
    required TResult Function(Cqrs_TodoQuery value) todoQuery,
  }) {
    return todoCommand(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommand value)? todoCommand,
    TResult? Function(Cqrs_TodoQuery value)? todoQuery,
  }) {
    return todoCommand?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommand value)? todoCommand,
    TResult Function(Cqrs_TodoQuery value)? todoQuery,
    required TResult orElse(),
  }) {
    if (todoCommand != null) {
      return todoCommand(this);
    }
    return orElse();
  }
}

abstract class Cqrs_TodoCommand extends Cqrs {
  const factory Cqrs_TodoCommand(final TodoCommand field0) =
      _$Cqrs_TodoCommandImpl;
  const Cqrs_TodoCommand._() : super._();

  @override
  TodoCommand get field0;

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Cqrs_TodoCommandImplCopyWith<_$Cqrs_TodoCommandImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Cqrs_TodoQueryImplCopyWith<$Res> {
  factory _$$Cqrs_TodoQueryImplCopyWith(_$Cqrs_TodoQueryImpl value,
          $Res Function(_$Cqrs_TodoQueryImpl) then) =
      __$$Cqrs_TodoQueryImplCopyWithImpl<$Res>;
  @useResult
  $Res call({TodoQuery field0});
}

/// @nodoc
class __$$Cqrs_TodoQueryImplCopyWithImpl<$Res>
    extends _$CqrsCopyWithImpl<$Res, _$Cqrs_TodoQueryImpl>
    implements _$$Cqrs_TodoQueryImplCopyWith<$Res> {
  __$$Cqrs_TodoQueryImplCopyWithImpl(
      _$Cqrs_TodoQueryImpl _value, $Res Function(_$Cqrs_TodoQueryImpl) _then)
      : super(_value, _then);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Cqrs_TodoQueryImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as TodoQuery,
    ));
  }
}

/// @nodoc

class _$Cqrs_TodoQueryImpl extends Cqrs_TodoQuery {
  const _$Cqrs_TodoQueryImpl(this.field0) : super._();

  @override
  final TodoQuery field0;

  @override
  String toString() {
    return 'Cqrs.todoQuery(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Cqrs_TodoQueryImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Cqrs_TodoQueryImplCopyWith<_$Cqrs_TodoQueryImpl> get copyWith =>
      __$$Cqrs_TodoQueryImplCopyWithImpl<_$Cqrs_TodoQueryImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(TodoCommand field0) todoCommand,
    required TResult Function(TodoQuery field0) todoQuery,
  }) {
    return todoQuery(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(TodoCommand field0)? todoCommand,
    TResult? Function(TodoQuery field0)? todoQuery,
  }) {
    return todoQuery?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(TodoCommand field0)? todoCommand,
    TResult Function(TodoQuery field0)? todoQuery,
    required TResult orElse(),
  }) {
    if (todoQuery != null) {
      return todoQuery(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommand value) todoCommand,
    required TResult Function(Cqrs_TodoQuery value) todoQuery,
  }) {
    return todoQuery(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommand value)? todoCommand,
    TResult? Function(Cqrs_TodoQuery value)? todoQuery,
  }) {
    return todoQuery?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommand value)? todoCommand,
    TResult Function(Cqrs_TodoQuery value)? todoQuery,
    required TResult orElse(),
  }) {
    if (todoQuery != null) {
      return todoQuery(this);
    }
    return orElse();
  }
}

abstract class Cqrs_TodoQuery extends Cqrs {
  const factory Cqrs_TodoQuery(final TodoQuery field0) = _$Cqrs_TodoQueryImpl;
  const Cqrs_TodoQuery._() : super._();

  @override
  TodoQuery get field0;

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Cqrs_TodoQueryImplCopyWith<_$Cqrs_TodoQueryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

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
