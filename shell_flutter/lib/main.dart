import 'package:flutter/material.dart';
import 'package:shell_flutter/state_handler.dart';

import 'bridge/frb_generated/domain/todo_list.dart';

Future<void> main() async {
  await StateHandler.createSingleton();
  runApp(const MyApp());
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
      home: MainScreen(title: 'Flutter-rust-bridge crux style Demo'),
    );
  }
}

class MainScreen extends StatelessWidget {
  MainScreen({super.key, required this.title});

  final String title;

  final TextEditingController textController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(title),
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
                    StateHandler.singleton
                        .processCommand(Command.addTodo(value));
                    textController.clear();
                  },
                ),
              ),
              ElevatedButton(
                onPressed: () {
                  StateHandler.singleton
                      .processCommand(Command_AddTodo(textController.text));
                  textController.clear();
                },
                child: const Text("Add Todo"),
              ),
            ],
          ),
          _buildTodoViewList(),
        ],
      ),
    );
  }

  Widget _buildTodoViewList() {
    // listens for the state stored in StateHandler
    return ValueListenableBuilder(
        valueListenable: StateHandler.singleton.todoListItems,
        builder: (context, todoListItems, _) {
          if (todoListItems.isEmpty) {
            return const Center(
              child: Text("No Todo's in the List!"),
            );
          } else {
            return Expanded(
              child: ListView.builder(
                scrollDirection: Axis.vertical,
                shrinkWrap: true,
                itemCount: todoListItems.length,
                itemBuilder: (context, index) {
                  return ListTile(
                      title: Row(
                    children: [
                      ElevatedButton(
                        child: const Icon(Icons.remove),
                        onPressed: () {
                          StateHandler.singleton
                              .processCommand(Command.removeTodo(index + 1));
                        },
                      ),
                      Text(' ${index + 1}.: ${todoListItems[index]}'),
                    ],
                  ));
                },
              ),
            );
          }
        });
  }
}
