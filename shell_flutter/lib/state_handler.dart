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
  // late final ValueNotifier<TodoListModel> todoListModel; // =
  // ValueNotifier(TodoListModel(items: VecStringImpl));

  // for more fine-granular UI updates, we create a listener for individual fields
  // of the TodoListModel.
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
    var effects = await todo_list_api.processCommand(command: command);
    _handleEffects(effects);
  }

  Future<void> processQuery(Query query) async {
    var effects = await todo_list_api.processQuery(query: query);
    _handleEffects(effects);
  }

  void _handleEffects(List<Effect> effects) {
    for (var effect in effects) {
      switch (effect) {
        // case Effect.renderTodoList():
        //   // update the value and trigger a UI repaint
        //   // note that only the reference is copied, not the whole list!
        //   StateHandler.singleton.todoListItems.value = effectValue.field0;
        //   break;
        // case Effect_RenderTodoList():
        case Effect_RenderTodoList():
          todoListItems.value = effect.field0;
        // TODO: Handle this case.
      }
    }
  }
}
