// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'effects.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Effect {
  TodoListModel get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(TodoListModel field0) renderTodoList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(TodoListModel field0)? renderTodoList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(TodoListModel field0)? renderTodoList,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Effect_RenderTodoList value) renderTodoList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Effect_RenderTodoList value)? renderTodoList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Effect_RenderTodoList value)? renderTodoList,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Create a copy of Effect
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $EffectCopyWith<Effect> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EffectCopyWith<$Res> {
  factory $EffectCopyWith(Effect value, $Res Function(Effect) then) =
      _$EffectCopyWithImpl<$Res, Effect>;
  @useResult
  $Res call({TodoListModel field0});
}

/// @nodoc
class _$EffectCopyWithImpl<$Res, $Val extends Effect>
    implements $EffectCopyWith<$Res> {
  _$EffectCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Effect
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_value.copyWith(
      field0: null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as TodoListModel,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$Effect_RenderTodoListImplCopyWith<$Res>
    implements $EffectCopyWith<$Res> {
  factory _$$Effect_RenderTodoListImplCopyWith(
          _$Effect_RenderTodoListImpl value,
          $Res Function(_$Effect_RenderTodoListImpl) then) =
      __$$Effect_RenderTodoListImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({TodoListModel field0});
}

/// @nodoc
class __$$Effect_RenderTodoListImplCopyWithImpl<$Res>
    extends _$EffectCopyWithImpl<$Res, _$Effect_RenderTodoListImpl>
    implements _$$Effect_RenderTodoListImplCopyWith<$Res> {
  __$$Effect_RenderTodoListImplCopyWithImpl(_$Effect_RenderTodoListImpl _value,
      $Res Function(_$Effect_RenderTodoListImpl) _then)
      : super(_value, _then);

  /// Create a copy of Effect
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Effect_RenderTodoListImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as TodoListModel,
    ));
  }
}

/// @nodoc

class _$Effect_RenderTodoListImpl extends Effect_RenderTodoList {
  const _$Effect_RenderTodoListImpl(this.field0) : super._();

  @override
  final TodoListModel field0;

  @override
  String toString() {
    return 'Effect.renderTodoList(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Effect_RenderTodoListImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Effect
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Effect_RenderTodoListImplCopyWith<_$Effect_RenderTodoListImpl>
      get copyWith => __$$Effect_RenderTodoListImplCopyWithImpl<
          _$Effect_RenderTodoListImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(TodoListModel field0) renderTodoList,
  }) {
    return renderTodoList(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(TodoListModel field0)? renderTodoList,
  }) {
    return renderTodoList?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(TodoListModel field0)? renderTodoList,
    required TResult orElse(),
  }) {
    if (renderTodoList != null) {
      return renderTodoList(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Effect_RenderTodoList value) renderTodoList,
  }) {
    return renderTodoList(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Effect_RenderTodoList value)? renderTodoList,
  }) {
    return renderTodoList?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Effect_RenderTodoList value)? renderTodoList,
    required TResult orElse(),
  }) {
    if (renderTodoList != null) {
      return renderTodoList(this);
    }
    return orElse();
  }
}

abstract class Effect_RenderTodoList extends Effect {
  const factory Effect_RenderTodoList(final TodoListModel field0) =
      _$Effect_RenderTodoListImpl;
  const Effect_RenderTodoList._() : super._();

  @override
  TodoListModel get field0;

  /// Create a copy of Effect
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Effect_RenderTodoListImplCopyWith<_$Effect_RenderTodoListImpl>
      get copyWith => throw _privateConstructorUsedError;
}
