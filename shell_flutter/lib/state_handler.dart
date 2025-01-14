import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shell_flutter/bridge/frb_generated/application/api/lifecycle.dart';
import 'package:shell_flutter/bridge/frb_generated/application/app_config.dart';
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
  final ValueNotifier<List<String>> todoListItems = ValueNotifier(List.empty());
  final ValueNotifier<String> todoListTitle =
      ValueNotifier("Click here to set a title");

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

    await LifecycleImpl.initialiseWithFilePersister(
        appConfig: AppConfigImpl(appStateUrl: stateFile.path));
    singleton = StateHandler._singletonConstructor();
    // initialise all Listeners with the loaded model
    // by calling the respective querries
    // the value is set by _handleEffects() automatically
    // load the todo List
    var effect = await const TodoListModelQuery.getAllTodos().process();
    // set it to the category
    TodoCategoryModelCommand.setTodoList(
            effect.first.field0 as TodoListModelLock)
        .process();
    // load the category with the todo list and update listener values
    singleton.handleEffects(
        const TodoCategoryModelQuery.getTodoCategoryModel().process());
    return singleton;
  }

  Future<void> handleEffects(Future<List<Effect>> effects) async {
    for (var effect in await effects) {
      switch (effect) {
        // update the value and trigger a UI repaint
        // we don't destinguish between whole list or just one item
        case Effect_TodoCategoryModelRenderTodoList():
          todoListItems.value = await effect.field0.lock.getTodosAsString();
          break;
        case Effect_TodoListModelRenderTodoList():
          todoListItems.value = await effect.field0.lock.getTodosAsString();
          break;
        case Effect_TodoListModelRenderTodoItem():
          // do nothing with it
          effect.field0;
          break;
        case Effect_TodoCategoryModelRenderTodoCategoryModel():
          todoListTitle.value = await effect.field0.lock.getTitle();
          todoListItems.value = await effect.field0.lock.getTodos();
          break;
        case Effect_TodoCategoryModelRenderTodoCategory():
          todoListTitle.value = effect.field0;
          break;
        case Effect_TodoCategoryModelUpdateTitel():
          todoListTitle.value = effect.field0;
          break;
      }
    }
  }
}
