// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'processing.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Cqrs {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) todoCommandAddTodo,
    required TResult Function(BigInt field0) todoCommandRemoveTodo,
    required TResult Function() todoCommandCleanList,
    required TResult Function() todoQueryAllTodos,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? todoCommandAddTodo,
    TResult? Function(BigInt field0)? todoCommandRemoveTodo,
    TResult? Function()? todoCommandCleanList,
    TResult? Function()? todoQueryAllTodos,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? todoCommandAddTodo,
    TResult Function(BigInt field0)? todoCommandRemoveTodo,
    TResult Function()? todoCommandCleanList,
    TResult Function()? todoQueryAllTodos,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommandAddTodo value) todoCommandAddTodo,
    required TResult Function(Cqrs_TodoCommandRemoveTodo value)
        todoCommandRemoveTodo,
    required TResult Function(Cqrs_TodoCommandCleanList value)
        todoCommandCleanList,
    required TResult Function(Cqrs_TodoQueryAllTodos value) todoQueryAllTodos,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult? Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult? Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult? Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
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
abstract class _$$Cqrs_TodoCommandAddTodoImplCopyWith<$Res> {
  factory _$$Cqrs_TodoCommandAddTodoImplCopyWith(
          _$Cqrs_TodoCommandAddTodoImpl value,
          $Res Function(_$Cqrs_TodoCommandAddTodoImpl) then) =
      __$$Cqrs_TodoCommandAddTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Cqrs_TodoCommandAddTodoImplCopyWithImpl<$Res>
    extends _$CqrsCopyWithImpl<$Res, _$Cqrs_TodoCommandAddTodoImpl>
    implements _$$Cqrs_TodoCommandAddTodoImplCopyWith<$Res> {
  __$$Cqrs_TodoCommandAddTodoImplCopyWithImpl(
      _$Cqrs_TodoCommandAddTodoImpl _value,
      $Res Function(_$Cqrs_TodoCommandAddTodoImpl) _then)
      : super(_value, _then);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Cqrs_TodoCommandAddTodoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Cqrs_TodoCommandAddTodoImpl extends Cqrs_TodoCommandAddTodo {
  const _$Cqrs_TodoCommandAddTodoImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Cqrs.todoCommandAddTodo(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Cqrs_TodoCommandAddTodoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Cqrs_TodoCommandAddTodoImplCopyWith<_$Cqrs_TodoCommandAddTodoImpl>
      get copyWith => __$$Cqrs_TodoCommandAddTodoImplCopyWithImpl<
          _$Cqrs_TodoCommandAddTodoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) todoCommandAddTodo,
    required TResult Function(BigInt field0) todoCommandRemoveTodo,
    required TResult Function() todoCommandCleanList,
    required TResult Function() todoQueryAllTodos,
  }) {
    return todoCommandAddTodo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? todoCommandAddTodo,
    TResult? Function(BigInt field0)? todoCommandRemoveTodo,
    TResult? Function()? todoCommandCleanList,
    TResult? Function()? todoQueryAllTodos,
  }) {
    return todoCommandAddTodo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? todoCommandAddTodo,
    TResult Function(BigInt field0)? todoCommandRemoveTodo,
    TResult Function()? todoCommandCleanList,
    TResult Function()? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoCommandAddTodo != null) {
      return todoCommandAddTodo(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommandAddTodo value) todoCommandAddTodo,
    required TResult Function(Cqrs_TodoCommandRemoveTodo value)
        todoCommandRemoveTodo,
    required TResult Function(Cqrs_TodoCommandCleanList value)
        todoCommandCleanList,
    required TResult Function(Cqrs_TodoQueryAllTodos value) todoQueryAllTodos,
  }) {
    return todoCommandAddTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult? Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult? Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult? Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
  }) {
    return todoCommandAddTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoCommandAddTodo != null) {
      return todoCommandAddTodo(this);
    }
    return orElse();
  }
}

abstract class Cqrs_TodoCommandAddTodo extends Cqrs {
  const factory Cqrs_TodoCommandAddTodo(final String field0) =
      _$Cqrs_TodoCommandAddTodoImpl;
  const Cqrs_TodoCommandAddTodo._() : super._();

  String get field0;

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Cqrs_TodoCommandAddTodoImplCopyWith<_$Cqrs_TodoCommandAddTodoImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Cqrs_TodoCommandRemoveTodoImplCopyWith<$Res> {
  factory _$$Cqrs_TodoCommandRemoveTodoImplCopyWith(
          _$Cqrs_TodoCommandRemoveTodoImpl value,
          $Res Function(_$Cqrs_TodoCommandRemoveTodoImpl) then) =
      __$$Cqrs_TodoCommandRemoveTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt field0});
}

/// @nodoc
class __$$Cqrs_TodoCommandRemoveTodoImplCopyWithImpl<$Res>
    extends _$CqrsCopyWithImpl<$Res, _$Cqrs_TodoCommandRemoveTodoImpl>
    implements _$$Cqrs_TodoCommandRemoveTodoImplCopyWith<$Res> {
  __$$Cqrs_TodoCommandRemoveTodoImplCopyWithImpl(
      _$Cqrs_TodoCommandRemoveTodoImpl _value,
      $Res Function(_$Cqrs_TodoCommandRemoveTodoImpl) _then)
      : super(_value, _then);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Cqrs_TodoCommandRemoveTodoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class _$Cqrs_TodoCommandRemoveTodoImpl extends Cqrs_TodoCommandRemoveTodo {
  const _$Cqrs_TodoCommandRemoveTodoImpl(this.field0) : super._();

  @override
  final BigInt field0;

  @override
  String toString() {
    return 'Cqrs.todoCommandRemoveTodo(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Cqrs_TodoCommandRemoveTodoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Cqrs_TodoCommandRemoveTodoImplCopyWith<_$Cqrs_TodoCommandRemoveTodoImpl>
      get copyWith => __$$Cqrs_TodoCommandRemoveTodoImplCopyWithImpl<
          _$Cqrs_TodoCommandRemoveTodoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) todoCommandAddTodo,
    required TResult Function(BigInt field0) todoCommandRemoveTodo,
    required TResult Function() todoCommandCleanList,
    required TResult Function() todoQueryAllTodos,
  }) {
    return todoCommandRemoveTodo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? todoCommandAddTodo,
    TResult? Function(BigInt field0)? todoCommandRemoveTodo,
    TResult? Function()? todoCommandCleanList,
    TResult? Function()? todoQueryAllTodos,
  }) {
    return todoCommandRemoveTodo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? todoCommandAddTodo,
    TResult Function(BigInt field0)? todoCommandRemoveTodo,
    TResult Function()? todoCommandCleanList,
    TResult Function()? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoCommandRemoveTodo != null) {
      return todoCommandRemoveTodo(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommandAddTodo value) todoCommandAddTodo,
    required TResult Function(Cqrs_TodoCommandRemoveTodo value)
        todoCommandRemoveTodo,
    required TResult Function(Cqrs_TodoCommandCleanList value)
        todoCommandCleanList,
    required TResult Function(Cqrs_TodoQueryAllTodos value) todoQueryAllTodos,
  }) {
    return todoCommandRemoveTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult? Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult? Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult? Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
  }) {
    return todoCommandRemoveTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoCommandRemoveTodo != null) {
      return todoCommandRemoveTodo(this);
    }
    return orElse();
  }
}

abstract class Cqrs_TodoCommandRemoveTodo extends Cqrs {
  const factory Cqrs_TodoCommandRemoveTodo(final BigInt field0) =
      _$Cqrs_TodoCommandRemoveTodoImpl;
  const Cqrs_TodoCommandRemoveTodo._() : super._();

  BigInt get field0;

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Cqrs_TodoCommandRemoveTodoImplCopyWith<_$Cqrs_TodoCommandRemoveTodoImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Cqrs_TodoCommandCleanListImplCopyWith<$Res> {
  factory _$$Cqrs_TodoCommandCleanListImplCopyWith(
          _$Cqrs_TodoCommandCleanListImpl value,
          $Res Function(_$Cqrs_TodoCommandCleanListImpl) then) =
      __$$Cqrs_TodoCommandCleanListImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Cqrs_TodoCommandCleanListImplCopyWithImpl<$Res>
    extends _$CqrsCopyWithImpl<$Res, _$Cqrs_TodoCommandCleanListImpl>
    implements _$$Cqrs_TodoCommandCleanListImplCopyWith<$Res> {
  __$$Cqrs_TodoCommandCleanListImplCopyWithImpl(
      _$Cqrs_TodoCommandCleanListImpl _value,
      $Res Function(_$Cqrs_TodoCommandCleanListImpl) _then)
      : super(_value, _then);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$Cqrs_TodoCommandCleanListImpl extends Cqrs_TodoCommandCleanList {
  const _$Cqrs_TodoCommandCleanListImpl() : super._();

  @override
  String toString() {
    return 'Cqrs.todoCommandCleanList()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Cqrs_TodoCommandCleanListImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) todoCommandAddTodo,
    required TResult Function(BigInt field0) todoCommandRemoveTodo,
    required TResult Function() todoCommandCleanList,
    required TResult Function() todoQueryAllTodos,
  }) {
    return todoCommandCleanList();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? todoCommandAddTodo,
    TResult? Function(BigInt field0)? todoCommandRemoveTodo,
    TResult? Function()? todoCommandCleanList,
    TResult? Function()? todoQueryAllTodos,
  }) {
    return todoCommandCleanList?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? todoCommandAddTodo,
    TResult Function(BigInt field0)? todoCommandRemoveTodo,
    TResult Function()? todoCommandCleanList,
    TResult Function()? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoCommandCleanList != null) {
      return todoCommandCleanList();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommandAddTodo value) todoCommandAddTodo,
    required TResult Function(Cqrs_TodoCommandRemoveTodo value)
        todoCommandRemoveTodo,
    required TResult Function(Cqrs_TodoCommandCleanList value)
        todoCommandCleanList,
    required TResult Function(Cqrs_TodoQueryAllTodos value) todoQueryAllTodos,
  }) {
    return todoCommandCleanList(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult? Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult? Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult? Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
  }) {
    return todoCommandCleanList?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoCommandCleanList != null) {
      return todoCommandCleanList(this);
    }
    return orElse();
  }
}

abstract class Cqrs_TodoCommandCleanList extends Cqrs {
  const factory Cqrs_TodoCommandCleanList() = _$Cqrs_TodoCommandCleanListImpl;
  const Cqrs_TodoCommandCleanList._() : super._();
}

/// @nodoc
abstract class _$$Cqrs_TodoQueryAllTodosImplCopyWith<$Res> {
  factory _$$Cqrs_TodoQueryAllTodosImplCopyWith(
          _$Cqrs_TodoQueryAllTodosImpl value,
          $Res Function(_$Cqrs_TodoQueryAllTodosImpl) then) =
      __$$Cqrs_TodoQueryAllTodosImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Cqrs_TodoQueryAllTodosImplCopyWithImpl<$Res>
    extends _$CqrsCopyWithImpl<$Res, _$Cqrs_TodoQueryAllTodosImpl>
    implements _$$Cqrs_TodoQueryAllTodosImplCopyWith<$Res> {
  __$$Cqrs_TodoQueryAllTodosImplCopyWithImpl(
      _$Cqrs_TodoQueryAllTodosImpl _value,
      $Res Function(_$Cqrs_TodoQueryAllTodosImpl) _then)
      : super(_value, _then);

  /// Create a copy of Cqrs
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$Cqrs_TodoQueryAllTodosImpl extends Cqrs_TodoQueryAllTodos {
  const _$Cqrs_TodoQueryAllTodosImpl() : super._();

  @override
  String toString() {
    return 'Cqrs.todoQueryAllTodos()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Cqrs_TodoQueryAllTodosImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) todoCommandAddTodo,
    required TResult Function(BigInt field0) todoCommandRemoveTodo,
    required TResult Function() todoCommandCleanList,
    required TResult Function() todoQueryAllTodos,
  }) {
    return todoQueryAllTodos();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? todoCommandAddTodo,
    TResult? Function(BigInt field0)? todoCommandRemoveTodo,
    TResult? Function()? todoCommandCleanList,
    TResult? Function()? todoQueryAllTodos,
  }) {
    return todoQueryAllTodos?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? todoCommandAddTodo,
    TResult Function(BigInt field0)? todoCommandRemoveTodo,
    TResult Function()? todoCommandCleanList,
    TResult Function()? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoQueryAllTodos != null) {
      return todoQueryAllTodos();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Cqrs_TodoCommandAddTodo value) todoCommandAddTodo,
    required TResult Function(Cqrs_TodoCommandRemoveTodo value)
        todoCommandRemoveTodo,
    required TResult Function(Cqrs_TodoCommandCleanList value)
        todoCommandCleanList,
    required TResult Function(Cqrs_TodoQueryAllTodos value) todoQueryAllTodos,
  }) {
    return todoQueryAllTodos(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult? Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult? Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult? Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
  }) {
    return todoQueryAllTodos?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Cqrs_TodoCommandAddTodo value)? todoCommandAddTodo,
    TResult Function(Cqrs_TodoCommandRemoveTodo value)? todoCommandRemoveTodo,
    TResult Function(Cqrs_TodoCommandCleanList value)? todoCommandCleanList,
    TResult Function(Cqrs_TodoQueryAllTodos value)? todoQueryAllTodos,
    required TResult orElse(),
  }) {
    if (todoQueryAllTodos != null) {
      return todoQueryAllTodos(this);
    }
    return orElse();
  }
}

abstract class Cqrs_TodoQueryAllTodos extends Cqrs {
  const factory Cqrs_TodoQueryAllTodos() = _$Cqrs_TodoQueryAllTodosImpl;
  const Cqrs_TodoQueryAllTodos._() : super._();
}
