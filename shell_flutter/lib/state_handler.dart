import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shell_flutter/bridge/frb_generated/application/api/todo_list_api.dart'
    as todo_list_api;
import 'package:shell_flutter/bridge/frb_generated/application/api/lifecycle.dart'
    as lifecycle;
import 'package:shell_flutter/bridge/frb_generated/domain/todo_list.dart';
import 'package:shell_flutter/bridge/frb_generated/frb_generated.dart';

/// This singleton handles the state, and all communication with the lower layers (implemented in Rust).
class StateHandler {
  // private constructor to disable the default constructor
  StateHandler._singletonConstructor();

  /// runtime error, if not initialized by calling _createSingleton()
  static late final StateHandler singleton;
  static bool isInitialised = false;

  /// ViewModels, observed by the UI
  // late ValueNotifier<TodoListModel> todoListModel;
  final ValueNotifier<TodoListModel> todoListModel =
      ValueNotifier(TodoListModel(items: List.empty()));

  // private Factory, so async can be used (not possible in a constructor or factory)
  // call only once to create the singleton
  static Future<StateHandler> createSingleton() async {
    if (isInitialised) {
      return singleton;
    }

    await RustLib.init();
    WidgetsFlutterBinding.ensureInitialized();
    final Directory appSupportDir = await getApplicationSupportDirectory();
    final stateFile = File('${appSupportDir.path}/app_state.bin');
    await lifecycle.setup(path: stateFile.path);
    // load the state
    await lifecycle.init();

    singleton = StateHandler._singletonConstructor();
    singleton.processQuery(Query.getModel);
    return singleton;
  }

  Future<void> processCommand(Command command) async {
    var effects = await todo_list_api.processCommand(command: command);
    _handleEffects(effects);
  }

  Future<void> processQuery(Query query) async {
    var effects = await todo_list_api.processQuery(query: query);
    _handleEffects(effects);
  }
}

void _handleEffects(List<Effect> effects) {
  for (var effect in effects) {
    switch (effect) {
      case Effect_Render effectRender:
        // update the value and trigger a UI repaint
        StateHandler.singleton.todoListModel.value = effectRender.field0;
      default:
        // No Effect to take care of - or worse unknown effect!
        //TODO handle gracefully
        stderr.writeln("received unknown Effect $effect\n");
        exit(1);
    }
  }
}
