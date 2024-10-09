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
/// Using ValueNotifiers it updates the UI on state changes
class StateHandler {
  // private constructor to disable the default constructor
  StateHandler._singletonConstructor();

  /// runtime error, if not initialized by calling _createSingleton()
  static late final StateHandler singleton;
  static bool isInitialised = false;

  /// ViewModels, observed by the UI

  // for more fine-granular UI updates, we create a listener for individual fields
  // of the TodoListModel.
  // TODO populate here already!
  final ValueNotifier<List<String>> todoListItems = ValueNotifier(List.empty());

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
    await lifecycle.Lifecycle.newInstance(path: stateFile.path);
    singleton = StateHandler._singletonConstructor();
    // initialise all Listeners with the loaded model
    // by calling the respective querries
    // the value is set by _handleEffects() automatically
    singleton.processQuery(Query.allTodos);
    return singleton;
  }

  Future<void> processCommand(Command command) async {
    var effects = todo_list_api.processCommand(command: command);
    await _handleEffects(await effects);
  }

  Future<void> processQuery(Query query) async {
    var effects = todo_list_api.processQuery(query: query);
    await _handleEffects(await effects);
  }

  Future<void> _handleEffects(List<Effect> effects) async {
    for (var effect in effects) {
      switch (effect) {
        // update the value and trigger a UI repaint
        case Effect_RenderTodoList():
          todoListItems.value = await effect.field0.getTodosAsString();
      }
    }
  }
}
