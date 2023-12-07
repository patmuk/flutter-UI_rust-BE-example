import 'package:flutter/material.dart';
import 'package:shell_flutter/ffi.dart';
import 'bridge_definitions.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a blue toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: MainScreen(title: 'Flutter-rust-bridge crux style Demo'),
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
  late ViewModel viewModel;

  // this triggers the button event in the rust lib
  Future<void> _processEvent(Event event) async {
    var effects = await api.processEvent(event: event);
    for (Effect effect in effects) {
      effect.when(render: (ViewModel viewModel) => setState(() {}));
    }
    // triggers a refresh
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
          // use 'setState' for triggering a refresh, which calling the future
          FutureBuilder(
            future: api.view(),
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
          ),
        ],
      ),
    );
  }
}
