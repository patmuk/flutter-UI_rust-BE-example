// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

// Static analysis wrongly picks the IO variant, thus ignore this
// ignore_for_file: argument_type_not_assignable

import 'application/api/lifecycle.dart';
import 'application/app_state.dart';
import 'dart:async';
import 'dart:convert';
import 'domain/todo_category.dart';
import 'domain/todo_list.dart';
import 'frb_generated.dart';
import 'lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_AppConfigImplPtr => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_AppStateImplPtr => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_LifecycleImplPtr => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_PathBufPtr => wire
      .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_ProcessingErrorPtr => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_TodoCategoryModelPtr => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_TodoListModelPtr => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  TodoCategoryModel
      dco_decode_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          dynamic raw);

  @protected
  TodoListModel
      dco_decode_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          dynamic raw);

  @protected
  AppConfigImpl
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          dynamic raw);

  @protected
  AppStateImpl
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          dynamic raw);

  @protected
  LifecycleImpl
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          dynamic raw);

  @protected
  PathBuf
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          dynamic raw);

  @protected
  ProcessingError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          dynamic raw);

  @protected
  TodoCategoryModel
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          dynamic raw);

  @protected
  TodoListModel
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          dynamic raw);

  @protected
  AppConfigImpl
      dco_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          dynamic raw);

  @protected
  AppConfigImpl
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          dynamic raw);

  @protected
  AppStateImpl
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          dynamic raw);

  @protected
  LifecycleImpl
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          dynamic raw);

  @protected
  PathBuf
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          dynamic raw);

  @protected
  TodoCategoryModel
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          dynamic raw);

  @protected
  TodoListModel
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          dynamic raw);

  @protected
  AppConfigImpl
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          dynamic raw);

  @protected
  AppStateImpl
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          dynamic raw);

  @protected
  LifecycleImpl
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          dynamic raw);

  @protected
  PathBuf
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          dynamic raw);

  @protected
  ProcessingError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          dynamic raw);

  @protected
  TodoCategoryModel
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          dynamic raw);

  @protected
  TodoListModel
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  AppConfig dco_decode_TraitDef_AppConfig(dynamic raw);

  @protected
  AppState dco_decode_TraitDef_AppState(dynamic raw);

  @protected
  Cqrs dco_decode_TraitDef_Cqrs(dynamic raw);

  @protected
  CqrsModel dco_decode_TraitDef_CqrsModel(dynamic raw);

  @protected
  CqrsModelLock dco_decode_TraitDef_CqrsModelLock(dynamic raw);

  @protected
  Lifecycle dco_decode_TraitDef_Lifecycle(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  TodoCategoryModelCommand dco_decode_box_autoadd_todo_category_model_command(
      dynamic raw);

  @protected
  TodoCategoryModelLock dco_decode_box_autoadd_todo_category_model_lock(
      dynamic raw);

  @protected
  TodoCategoryModelQuery dco_decode_box_autoadd_todo_category_model_query(
      dynamic raw);

  @protected
  TodoItem dco_decode_box_autoadd_todo_item(dynamic raw);

  @protected
  TodoListModelCommand dco_decode_box_autoadd_todo_list_model_command(
      dynamic raw);

  @protected
  TodoListModelLock dco_decode_box_autoadd_todo_list_model_lock(dynamic raw);

  @protected
  TodoListModelQuery dco_decode_box_autoadd_todo_list_model_query(dynamic raw);

  @protected
  Effect dco_decode_effect(dynamic raw);

  @protected
  List<String> dco_decode_list_String(dynamic raw);

  @protected
  List<Effect> dco_decode_list_effect(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  String? dco_decode_opt_String(dynamic raw);

  @protected
  TodoCategoryModelCommand dco_decode_todo_category_model_command(dynamic raw);

  @protected
  TodoCategoryModelLock dco_decode_todo_category_model_lock(dynamic raw);

  @protected
  TodoCategoryModelQuery dco_decode_todo_category_model_query(dynamic raw);

  @protected
  TodoItem dco_decode_todo_item(dynamic raw);

  @protected
  TodoListModelCommand dco_decode_todo_list_model_command(dynamic raw);

  @protected
  TodoListModelLock dco_decode_todo_list_model_lock(dynamic raw);

  @protected
  TodoListModelQuery dco_decode_todo_list_model_query(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  BigInt dco_decode_usize(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  TodoCategoryModel
      sse_decode_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          SseDeserializer deserializer);

  @protected
  TodoListModel
      sse_decode_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          SseDeserializer deserializer);

  @protected
  AppConfigImpl
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          SseDeserializer deserializer);

  @protected
  AppStateImpl
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          SseDeserializer deserializer);

  @protected
  LifecycleImpl
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          SseDeserializer deserializer);

  @protected
  PathBuf
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          SseDeserializer deserializer);

  @protected
  ProcessingError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          SseDeserializer deserializer);

  @protected
  TodoCategoryModel
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          SseDeserializer deserializer);

  @protected
  TodoListModel
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          SseDeserializer deserializer);

  @protected
  AppConfigImpl
      sse_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          SseDeserializer deserializer);

  @protected
  AppConfigImpl
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          SseDeserializer deserializer);

  @protected
  AppStateImpl
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          SseDeserializer deserializer);

  @protected
  LifecycleImpl
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          SseDeserializer deserializer);

  @protected
  PathBuf
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          SseDeserializer deserializer);

  @protected
  TodoCategoryModel
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          SseDeserializer deserializer);

  @protected
  TodoListModel
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          SseDeserializer deserializer);

  @protected
  AppConfigImpl
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          SseDeserializer deserializer);

  @protected
  AppStateImpl
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          SseDeserializer deserializer);

  @protected
  LifecycleImpl
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          SseDeserializer deserializer);

  @protected
  PathBuf
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          SseDeserializer deserializer);

  @protected
  ProcessingError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          SseDeserializer deserializer);

  @protected
  TodoCategoryModel
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          SseDeserializer deserializer);

  @protected
  TodoListModel
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  TodoCategoryModelCommand sse_decode_box_autoadd_todo_category_model_command(
      SseDeserializer deserializer);

  @protected
  TodoCategoryModelLock sse_decode_box_autoadd_todo_category_model_lock(
      SseDeserializer deserializer);

  @protected
  TodoCategoryModelQuery sse_decode_box_autoadd_todo_category_model_query(
      SseDeserializer deserializer);

  @protected
  TodoItem sse_decode_box_autoadd_todo_item(SseDeserializer deserializer);

  @protected
  TodoListModelCommand sse_decode_box_autoadd_todo_list_model_command(
      SseDeserializer deserializer);

  @protected
  TodoListModelLock sse_decode_box_autoadd_todo_list_model_lock(
      SseDeserializer deserializer);

  @protected
  TodoListModelQuery sse_decode_box_autoadd_todo_list_model_query(
      SseDeserializer deserializer);

  @protected
  Effect sse_decode_effect(SseDeserializer deserializer);

  @protected
  List<String> sse_decode_list_String(SseDeserializer deserializer);

  @protected
  List<Effect> sse_decode_list_effect(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer);

  @protected
  TodoCategoryModelCommand sse_decode_todo_category_model_command(
      SseDeserializer deserializer);

  @protected
  TodoCategoryModelLock sse_decode_todo_category_model_lock(
      SseDeserializer deserializer);

  @protected
  TodoCategoryModelQuery sse_decode_todo_category_model_query(
      SseDeserializer deserializer);

  @protected
  TodoItem sse_decode_todo_item(SseDeserializer deserializer);

  @protected
  TodoListModelCommand sse_decode_todo_list_model_command(
      SseDeserializer deserializer);

  @protected
  TodoListModelLock sse_decode_todo_list_model_lock(
      SseDeserializer deserializer);

  @protected
  TodoListModelQuery sse_decode_todo_list_model_query(
      SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void
      sse_encode_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          TodoCategoryModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          TodoListModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          AppConfigImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          AppStateImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          LifecycleImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          PathBuf self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          ProcessingError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          TodoCategoryModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          TodoListModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          AppConfigImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          AppConfigImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          AppStateImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          LifecycleImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          PathBuf self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          TodoCategoryModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          TodoListModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          AppConfigImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          AppStateImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          LifecycleImpl self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          PathBuf self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          ProcessingError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          TodoCategoryModel self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          TodoListModel self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_category_model_command(
      TodoCategoryModelCommand self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_category_model_lock(
      TodoCategoryModelLock self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_category_model_query(
      TodoCategoryModelQuery self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_item(
      TodoItem self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_list_model_command(
      TodoListModelCommand self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_list_model_lock(
      TodoListModelLock self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_todo_list_model_query(
      TodoListModelQuery self, SseSerializer serializer);

  @protected
  void sse_encode_effect(Effect self, SseSerializer serializer);

  @protected
  void sse_encode_list_String(List<String> self, SseSerializer serializer);

  @protected
  void sse_encode_list_effect(List<Effect> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer);

  @protected
  void sse_encode_todo_category_model_command(
      TodoCategoryModelCommand self, SseSerializer serializer);

  @protected
  void sse_encode_todo_category_model_lock(
      TodoCategoryModelLock self, SseSerializer serializer);

  @protected
  void sse_encode_todo_category_model_query(
      TodoCategoryModelQuery self, SseSerializer serializer);

  @protected
  void sse_encode_todo_item(TodoItem self, SseSerializer serializer);

  @protected
  void sse_encode_todo_list_model_command(
      TodoListModelCommand self, SseSerializer serializer);

  @protected
  void sse_encode_todo_list_model_lock(
      TodoListModelLock self, SseSerializer serializer);

  @protected
  void sse_encode_todo_list_model_query(
      TodoListModelQuery self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  RustLibWire.fromExternalLibrary(ExternalLibrary lib);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
              ptr);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
              ptr);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
              ptr);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
              ptr);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
              ptr);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
              ptr);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          int ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          int ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
              ptr);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
extension type RustLibWasmModule._(JSObject _) implements JSObject {
  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppConfigImpl(
          int ptr);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerAppStateImpl(
          int ptr);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerLifecycleImpl(
          int ptr);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(
          int ptr);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerProcessingError(
          int ptr);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoCategoryModel(
          int ptr);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          int ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTodoListModel(
          int ptr);
}
