// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_definitions.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$Effect {
  ViewModel get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ViewModel field0) render,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ViewModel field0)? render,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ViewModel field0)? render,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Effect_Render value) render,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Effect_Render value)? render,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Effect_Render value)? render,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $EffectCopyWith<Effect> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EffectCopyWith<$Res> {
  factory $EffectCopyWith(Effect value, $Res Function(Effect) then) = _$EffectCopyWithImpl<$Res, Effect>;
  @useResult
  $Res call({ViewModel field0});
}

/// @nodoc
class _$EffectCopyWithImpl<$Res, $Val extends Effect> implements $EffectCopyWith<$Res> {
  _$EffectCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_value.copyWith(
      field0: null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ViewModel,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$Effect_RenderCopyWith<$Res> implements $EffectCopyWith<$Res> {
  factory _$$Effect_RenderCopyWith(_$Effect_Render value, $Res Function(_$Effect_Render) then) =
      __$$Effect_RenderCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({ViewModel field0});
}

/// @nodoc
class __$$Effect_RenderCopyWithImpl<$Res> extends _$EffectCopyWithImpl<$Res, _$Effect_Render>
    implements _$$Effect_RenderCopyWith<$Res> {
  __$$Effect_RenderCopyWithImpl(_$Effect_Render _value, $Res Function(_$Effect_Render) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Effect_Render(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ViewModel,
    ));
  }
}

/// @nodoc

class _$Effect_Render implements Effect_Render {
  const _$Effect_Render(this.field0);

  @override
  final ViewModel field0;

  @override
  String toString() {
    return 'Effect.render(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Effect_Render &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Effect_RenderCopyWith<_$Effect_Render> get copyWith =>
      __$$Effect_RenderCopyWithImpl<_$Effect_Render>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ViewModel field0) render,
  }) {
    return render(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ViewModel field0)? render,
  }) {
    return render?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ViewModel field0)? render,
    required TResult orElse(),
  }) {
    if (render != null) {
      return render(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Effect_Render value) render,
  }) {
    return render(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Effect_Render value)? render,
  }) {
    return render?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Effect_Render value)? render,
    required TResult orElse(),
  }) {
    if (render != null) {
      return render(this);
    }
    return orElse();
  }
}

abstract class Effect_Render implements Effect {
  const factory Effect_Render(final ViewModel field0) = _$Effect_Render;

  @override
  ViewModel get field0;
  @override
  @JsonKey(ignore: true)
  _$$Effect_RenderCopyWith<_$Effect_Render> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Event {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(int field0) removeTodo,
    required TResult Function() cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(int field0)? removeTodo,
    TResult? Function()? cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(int field0)? removeTodo,
    TResult Function()? cleanList,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Event_AddTodo value) addTodo,
    required TResult Function(Event_RemoveTodo value) removeTodo,
    required TResult Function(Event_CleanList value) cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Event_AddTodo value)? addTodo,
    TResult? Function(Event_RemoveTodo value)? removeTodo,
    TResult? Function(Event_CleanList value)? cleanList,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Event_AddTodo value)? addTodo,
    TResult Function(Event_RemoveTodo value)? removeTodo,
    TResult Function(Event_CleanList value)? cleanList,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EventCopyWith<$Res> {
  factory $EventCopyWith(Event value, $Res Function(Event) then) = _$EventCopyWithImpl<$Res, Event>;
}

/// @nodoc
class _$EventCopyWithImpl<$Res, $Val extends Event> implements $EventCopyWith<$Res> {
  _$EventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Event_AddTodoCopyWith<$Res> {
  factory _$$Event_AddTodoCopyWith(_$Event_AddTodo value, $Res Function(_$Event_AddTodo) then) =
      __$$Event_AddTodoCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Event_AddTodoCopyWithImpl<$Res> extends _$EventCopyWithImpl<$Res, _$Event_AddTodo>
    implements _$$Event_AddTodoCopyWith<$Res> {
  __$$Event_AddTodoCopyWithImpl(_$Event_AddTodo _value, $Res Function(_$Event_AddTodo) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Event_AddTodo(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Event_AddTodo implements Event_AddTodo {
  const _$Event_AddTodo(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Event.addTodo(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Event_AddTodo &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Event_AddTodoCopyWith<_$Event_AddTodo> get copyWith =>
      __$$Event_AddTodoCopyWithImpl<_$Event_AddTodo>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(int field0) removeTodo,
    required TResult Function() cleanList,
  }) {
    return addTodo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(int field0)? removeTodo,
    TResult? Function()? cleanList,
  }) {
    return addTodo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(int field0)? removeTodo,
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
    required TResult Function(Event_AddTodo value) addTodo,
    required TResult Function(Event_RemoveTodo value) removeTodo,
    required TResult Function(Event_CleanList value) cleanList,
  }) {
    return addTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Event_AddTodo value)? addTodo,
    TResult? Function(Event_RemoveTodo value)? removeTodo,
    TResult? Function(Event_CleanList value)? cleanList,
  }) {
    return addTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Event_AddTodo value)? addTodo,
    TResult Function(Event_RemoveTodo value)? removeTodo,
    TResult Function(Event_CleanList value)? cleanList,
    required TResult orElse(),
  }) {
    if (addTodo != null) {
      return addTodo(this);
    }
    return orElse();
  }
}

abstract class Event_AddTodo implements Event {
  const factory Event_AddTodo(final String field0) = _$Event_AddTodo;

  String get field0;
  @JsonKey(ignore: true)
  _$$Event_AddTodoCopyWith<_$Event_AddTodo> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Event_RemoveTodoCopyWith<$Res> {
  factory _$$Event_RemoveTodoCopyWith(_$Event_RemoveTodo value, $Res Function(_$Event_RemoveTodo) then) =
      __$$Event_RemoveTodoCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$Event_RemoveTodoCopyWithImpl<$Res> extends _$EventCopyWithImpl<$Res, _$Event_RemoveTodo>
    implements _$$Event_RemoveTodoCopyWith<$Res> {
  __$$Event_RemoveTodoCopyWithImpl(_$Event_RemoveTodo _value, $Res Function(_$Event_RemoveTodo) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Event_RemoveTodo(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Event_RemoveTodo implements Event_RemoveTodo {
  const _$Event_RemoveTodo(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'Event.removeTodo(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Event_RemoveTodo &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Event_RemoveTodoCopyWith<_$Event_RemoveTodo> get copyWith =>
      __$$Event_RemoveTodoCopyWithImpl<_$Event_RemoveTodo>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(int field0) removeTodo,
    required TResult Function() cleanList,
  }) {
    return removeTodo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(int field0)? removeTodo,
    TResult? Function()? cleanList,
  }) {
    return removeTodo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(int field0)? removeTodo,
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
    required TResult Function(Event_AddTodo value) addTodo,
    required TResult Function(Event_RemoveTodo value) removeTodo,
    required TResult Function(Event_CleanList value) cleanList,
  }) {
    return removeTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Event_AddTodo value)? addTodo,
    TResult? Function(Event_RemoveTodo value)? removeTodo,
    TResult? Function(Event_CleanList value)? cleanList,
  }) {
    return removeTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Event_AddTodo value)? addTodo,
    TResult Function(Event_RemoveTodo value)? removeTodo,
    TResult Function(Event_CleanList value)? cleanList,
    required TResult orElse(),
  }) {
    if (removeTodo != null) {
      return removeTodo(this);
    }
    return orElse();
  }
}

abstract class Event_RemoveTodo implements Event {
  const factory Event_RemoveTodo(final int field0) = _$Event_RemoveTodo;

  int get field0;
  @JsonKey(ignore: true)
  _$$Event_RemoveTodoCopyWith<_$Event_RemoveTodo> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Event_CleanListCopyWith<$Res> {
  factory _$$Event_CleanListCopyWith(_$Event_CleanList value, $Res Function(_$Event_CleanList) then) =
      __$$Event_CleanListCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Event_CleanListCopyWithImpl<$Res> extends _$EventCopyWithImpl<$Res, _$Event_CleanList>
    implements _$$Event_CleanListCopyWith<$Res> {
  __$$Event_CleanListCopyWithImpl(_$Event_CleanList _value, $Res Function(_$Event_CleanList) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Event_CleanList implements Event_CleanList {
  const _$Event_CleanList();

  @override
  String toString() {
    return 'Event.cleanList()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Event_CleanList);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) addTodo,
    required TResult Function(int field0) removeTodo,
    required TResult Function() cleanList,
  }) {
    return cleanList();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? addTodo,
    TResult? Function(int field0)? removeTodo,
    TResult? Function()? cleanList,
  }) {
    return cleanList?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? addTodo,
    TResult Function(int field0)? removeTodo,
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
    required TResult Function(Event_AddTodo value) addTodo,
    required TResult Function(Event_RemoveTodo value) removeTodo,
    required TResult Function(Event_CleanList value) cleanList,
  }) {
    return cleanList(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Event_AddTodo value)? addTodo,
    TResult? Function(Event_RemoveTodo value)? removeTodo,
    TResult? Function(Event_CleanList value)? cleanList,
  }) {
    return cleanList?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Event_AddTodo value)? addTodo,
    TResult Function(Event_RemoveTodo value)? removeTodo,
    TResult Function(Event_CleanList value)? cleanList,
    required TResult orElse(),
  }) {
    if (cleanList != null) {
      return cleanList(this);
    }
    return orElse();
  }
}

abstract class Event_CleanList implements Event {
  const factory Event_CleanList() = _$Event_CleanList;
}
