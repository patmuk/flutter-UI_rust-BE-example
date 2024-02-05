import 'dart:io';

import 'package:flutter/material.dart';

import 'package:shell_flutter/bridge/frb_generated/api/lifecycle.dart'
    as lifecycle;
import 'package:shell_flutter/bridge/frb_generated/api/todo_list_api.dart'
    as todo_list_api;
import 'bridge/frb_generated/todo_list.dart';
import 'package:shell_flutter/bridge/frb_generated/frb_generated.dart';

import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();
  final Directory appSupportDir = await getApplicationSupportDirectory();
  final stateFile = File('${appSupportDir.path}/app_state.bin');
  runApp(const MyApp());
  await lifecycle.setup(path: stateFile.path);
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MainScreen(title: 'Flutter-rust-bridge crux style Demo'),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({super.key, required this.title});

  final String title;

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  late todo_list_api.ViewModel viewModel;

  // this triggers the button event in the rust lib
  Future<void> _processEvent(Event event) async {
    var effects = await todo_list_api.processEvent(event: event);
    for (Effect effect in effects) {
      effect.when(
          render: (todo_list_api.ViewModel viewModel) => setState(() {
                // triggers a refresh
                // setting viewModel is only needed, in no FutureBuilder to execute api.view() would be called
                this.viewModel = viewModel;
              }));
    }
  }

  final TextEditingController textController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Column(
        children: [
          Text(
            "---=== TODO LIST ===---",
            style: Theme.of(context).textTheme.headlineMedium,
          ),
          Row(
            children: [
              Expanded(
                child: TextField(
                  keyboardType: TextInputType.text,
                  controller: textController,
                  maxLines: null,
                  onSubmitted: (value) {
                    // Add your action here
                    _processEvent(Event.addTodo(value));
                    textController.clear();
                  },
                ),
              ),
              ElevatedButton(
                onPressed: () {
                  _processEvent(Event_AddTodo(textController.text));
                  textController.clear();
                },
                child: const Text("Add Todo"),
              ),
            ],
          ),
          _buildTodoViewList_withFuture(),
        ],
      ),
    );
  }

  Widget _buildTodoViewList_withFuture() {
    // use 'setState' for triggering a refresh, which triggers calling the future
    // note that calling api.view() is for demonstration only, as setState already
    // updates the view model
    return FutureBuilder(
      future: todo_list_api.view(),
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          viewModel = snapshot.data!;
          if (viewModel.count == 0) {
            return const Center(
              child: Text("No Todo's in the List!"),
            );
          } else {
            return Expanded(
              child: ListView.builder(
                scrollDirection: Axis.vertical,
                shrinkWrap: true,
                itemCount: viewModel.count,
                itemBuilder: (context, index) {
                  return ListTile(
                      title: Row(
                    children: [
                      ElevatedButton(
                        child: const Icon(Icons.remove),
                        onPressed: () {
                          _processEvent(Event.removeTodo(index + 1));
                        },
                      ),
                      Text(' ${index + 1}.: ${viewModel.items[index]}'),
                    ],
                  ));
                },
              ),
            );
          }
        } else if (snapshot.hasError) {
          return Center(
            child: Text('Error: ${snapshot.error}'),
          );
        } else {
          return const Center(
            child: CircularProgressIndicator(),
          );
        }
      },
    );
  }
}
