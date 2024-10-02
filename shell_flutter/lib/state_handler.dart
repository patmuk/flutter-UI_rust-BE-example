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

  // the app's state
  static late final app_state_ref;

  /// ViewModels, observed by the UI
  // for completeness, this is the whole model - not used in the UI
  late final ValueNotifier<TodoListModel> todoListModel; // =
  // ValueNotifier(TodoListModel(items: VecStringImpl));

  // for more fine-granular UI updates, we create a listener for individual fields
  // of the TodoListModel.
  // final ValueNotifier<List<String>> todoListItems = ValueNotifier(TodoListModel.);

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
    // initialise all Listeners with the loaded model
    // by calling the respective querries
    // the value is set by _handleEffects() automatically
    var appState = await lifecycle.getAppState();
    singleton.processQuery(Query.getModel);

    // var appConfigRef = await lifecycle.getAppConfigRef();
    // appConfigRef.appConfig.appStateFilePath;
    // var appState = await lifecycle.getAppState();
    // appStateRef.lock;
    // app_state_lock =
    //     await AppStateLock.loadOrNew(appConfig: appConfigRef.appConfig);

    return singleton;
  }

  Future<void> processCommand(Command command) async {
    var appState = await lifecycle.getAppState();
    await lifecycle.updateAppState(appState);
    var effects = await todo_list_api.processCommand(command: command);
    _handleEffects(effects);
  }

  Future<void> processQuery(Query query) async {
    var effects = await todo_list_api.processQuery(query: query);
    _handleEffects(effects);
  }
  // Future<void> processCommand(Command command) async {
  //   var effects = await todo_list_api.processCommand(command: command);
  //   _handleEffects(effects);
  // }

  // Future<void> processQuery(Query query) async {
  //   var effects = await todo_list_api.processQuery(query: query);
  //   _handleEffects(effects);
  // }
}

void _handleEffects(List<Effect> effects) {
  for (var effect in effects) {
    // switch (effect) {
    //   case Effect.render:
    //     // update the value and trigger a UI repaint
    //     // note that only the reference is copied, not the whole list!

    //     // var lock = lifecycle.getState();
    //     // StateHandler.singleton.todoListModel.value = effectValue.field0;
    //     // as the fields might have changed, their Listeners need to be updated as well!
    //     StateHandler.singleton.todoListItems.value = effectValue.field0.items;
    //     break;
    //   case Effect.renderTodoList:
    //     // update the value and trigger a UI repaint
    //     // note that only the reference is copied, not the whole list!
    //     StateHandler.singleton.todoListItems.value = effectValue.field0;
    //     break;
    // }
  }
}
