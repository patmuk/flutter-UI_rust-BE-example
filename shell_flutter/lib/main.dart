import 'package:flutter/material.dart';
import 'package:shell_flutter/bridge/frb_generated/application/api/lifecycle.dart';
import 'package:shell_flutter/state_handler.dart';

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
  final TextEditingController titleController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(title),
      ),
      body: Column(
        children: [
          ValueListenableBuilder(
              valueListenable: StateHandler.singleton.todoListTitle,
              builder: (context, todoListTitle, _) {
                return Center(
                    child: TextField(
                  decoration: InputDecoration(
                    border:
                        const UnderlineInputBorder(borderSide: BorderSide.none),
                    label: Center(
                      child: Text(
                        '---=== $todoListTitle ===---',
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                    ),
                    hintText: 'Enter a new title',
                    hintStyle: Theme.of(context).textTheme.labelLarge,
                  ),
                  keyboardType: TextInputType.text,
                  controller: titleController,
                  maxLines: null,
                  onSubmitted: (value) {
                    // Add your action here
                    StateHandler.singleton.handleEffects(
                      TodoCategoryModelCommand.updateTitle(value).process(),
                    );
                    titleController.clear();
                  },
                ));
              }),
          Row(
            children: [
              Expanded(
                child: TextField(
                  keyboardType: TextInputType.text,
                  controller: textController,
                  maxLines: null,
                  onSubmitted: (value) {
                    // Add your action here
                    StateHandler.singleton.handleEffects(
                        TodoListModelCommand.addTodo(value).process());
                    textController.clear();
                  },
                ),
              ),
              ElevatedButton(
                onPressed: () {
                  StateHandler.singleton.handleEffects(
                      TodoListModelCommand.addTodo(textController.text)
                          .process());
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
                          StateHandler.singleton.handleEffects(
                              TodoListModelCommand.removeTodo(
                                      BigInt.from(index + 1))
                                  .process());
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
