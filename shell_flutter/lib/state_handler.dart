import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:shell_flutter/bridge/frb_generated/application/state.dart';
import 'package:shell_flutter/bridge/frb_generated/frb_generated.dart';

/// This singleton handles the state, and all communication with the lower layers (implemented in Rust).
class StateHandler {
  // private constructor to disable the default constructor
  StateHandler._singletonConstructor(this.appState, Heavy? lastItem) {
    items = ValueNotifier(appState.items);
    this.lastItem = ValueNotifier(lastItem);
  }

  /// runtime error, if not initialized by calling _createSingleton()
  static late final StateHandler _singleton;
  static bool _isInitialised = false;

  // the app's state
  late final AppState appState;

  /// ViewModels, observed by the UI
  // for completeness, this is the whole model - not used in the UI
  // late final ValueNotifier<List<Heavy>> items = ValueNotifier(List.empty());
  // for more fine-granular UI updates, we create a listener for individual fields
  // of the TodoListModel.
  late final ValueNotifier<List<Heavy>> items;
  late final ValueNotifier<Heavy?> lastItem;

  // private Factory, so async can be used (not possible in a constructor or factory)
  // call only once to create the singleton

  static Future<StateHandler> init() async {
    return getSingleton();
  }

  static Future<StateHandler> getSingleton() async {
    if (_isInitialised) {
      return _singleton;
    }

    await RustLib.init();
    WidgetsFlutterBinding.ensureInitialized();
    // final Directory appSupportDir = await getApplicationSupportDirectory();
    // final stateFile = File('${appSupportDir.path}/app_state.bin');
    // await lifecycle.setup(path: stateFile.path);
    // // load the state
    // await lifecycle.init();
    var appState = await AppState.newInstance();
    var lastItem = await appState.getLastItem();
    print("got last item ${lastItem?.heavy}");

    _singleton = StateHandler._singletonConstructor(appState, lastItem);
    _isInitialised = true;
    // initialise all Listeners with the loaded model
    // by calling the respective querries
    // the value is set by _handleEffects() automatically
    // singleton.processQuery(Query.getModel);

    // var appConfigRef = await lifecycle.getAppConfigRef();
    // appConfigRef.appConfig.appStateFilePath;
    return _singleton;
  }

  Future<void> addItem({required int item}) async {
    await appState.addItem(item: item);
    //trigger rebuilds
    items.value = appState.items;
    lastItem.value = await appState.getLastItem();
    print("added item $item, list is now ${itemsToString()}\n");
  }

  Future<void> removeItem({required int index}) async {
    await appState.removeItem(index: index);
    //trigger rebuilds
    items.value = appState.items;
    lastItem.value = await appState.getLastItem();
    print("removed item at index $index, list is now ${itemsToString()}\n");
  }

  String itemsToString() {
    return appState.items.map((item) => item.heavy.toString()).join(", ");
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

// void _handleEffects(List<Effect> effects) {
//   for (var effect in effects) {
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
  // }
// }
