import 'package:flutter/material.dart';
import 'package:shell_flutter/bridge/frb_generated/application/state_test.dart';
import 'package:shell_flutter/bridge/frb_generated/frb_generated.dart';

var appState;
Future<void> main() async {
  await RustLib.init();
  appState = await AppState.newInstance();
  await appState.addItem(42);
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
      home: MainScreen(title: 'minimal state handling sample'),
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
            "---=== Handling a heavy item list simulation ===---",
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
                    // StateHandler.singleton
                    //     .processCommand(Command.addTodo(value));
                    textController.clear();
                  },
                ),
              ),
              ElevatedButton(
                onPressed: () {
                  // StateHandler.singleton
                  //     .processCommand(Command_AddTodo(textController.text));
                  textController.clear();
                },
                child: const Text("Add Todo"),
              ),
            ],
          ),
          _buildTodoViewList(context),
        ],
      ),
    );
  }

  Widget _buildTodoViewList(BuildContext context) {
    return DefaultTextStyle(
      style: Theme.of(context).textTheme.displayMedium!,
      textAlign: TextAlign.center,
      child: FutureBuilder<String>(
        future: appState
            .getFirstItem(), // a previously-obtained Future<String> or null
        builder: (BuildContext context, AsyncSnapshot<String> snapshot) {
          List<Widget> children;
          if (snapshot.hasData) {
            children = <Widget>[
              const Icon(
                Icons.check_circle_outline,
                color: Colors.green,
                size: 60,
              ),
              Padding(
                padding: const EdgeInsets.only(top: 16),
                child: Text('Result: ${snapshot.data}'),
              ),
            ];
          } else if (snapshot.hasError) {
            children = <Widget>[
              const Icon(
                Icons.error_outline,
                color: Colors.red,
                size: 60,
              ),
              Padding(
                padding: const EdgeInsets.only(top: 16),
                child: Text('Error: ${snapshot.error}'),
              ),
            ];
          } else {
            children = const <Widget>[
              SizedBox(
                width: 60,
                height: 60,
                child: CircularProgressIndicator(),
              ),
              Padding(
                padding: EdgeInsets.only(top: 16),
                child: Text('Awaiting result...'),
              ),
            ];
          }
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: children,
            ),
          );
        },
      ),
    );
  }

  // Widget org_buildTodoViewList() {
  //   // listens for the state stored in StateHandler
  //   return ValueListenableBuilder(
  //       valueListenable: StateHandler.singleton.todoListItems,
  //       builder: (context, todoListItems, _) {
  //         if (todoListItems.isEmpty) {
  //           return const Center(
  //             child: Text("No Todo's in the List!"),
  //           );
  //         } else {
  //           return Expanded(
  //             child: ListView.builder(
  //               scrollDirection: Axis.vertical,
  //               shrinkWrap: true,
  //               itemCount: todoListItems.length,
  //               itemBuilder: (context, index) {
  //                 return ListTile(
  //                     title: Row(
  //                   children: [
  //                     ElevatedButton(
  //                       child: const Icon(Icons.remove),
  //                       onPressed: () {
  //                         StateHandler.singleton
  //                             .processCommand(Command.removeTodo(index + 1));
  //                       },
  //                     ),
  //                     Text(' ${index + 1}.: ${todoListItems[index]}'),
  //                   ],
  //                 ));
  //               },
  //             ),
  //           );
  //         }
  //       });
  // }
}
