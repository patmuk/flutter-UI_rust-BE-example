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

class MainScreen extends StatelessWidget {
  MainScreen({super.key, required this.title});

  final String title;

  // this triggers the button event in the rust lib
  void _addTodo(String todo) {
    api.processEvent(event: Event_AddTodo(todo));
    setState() {}
  }

  final TextEditingController textController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.

    return Scaffold(
      appBar: AppBar(
        // TRY THIS: Try changing the color here to a specific color (to
        // Colors.amber, perhaps?) and trigger a hot reload to see the AppBar
        // change color while the other colors stay the same.
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(title),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: Column(
          // Column is also a layout widget. It takes a list of children and
          // arranges them vertically. By default, it sizes itself to fit its
          // children horizontally, and tries to be as tall as its parent.
          //
          // Column has various properties to control how it sizes itself and
          // how it positions its children. Here we use mainAxisAlignment to
          // center the children vertically; the main axis here is the vertical
          // axis because Columns are vertical (the cross axis would be
          // horizontal).
          //
          // TRY THIS: Invoke "debug painting" (choose the "Toggle Debug Paint"
          // action in the IDE, or press "p" in the console), to see the
          // wireframe for each widget.
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              "---=== TODO LIST ===---",
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            FutureBuilder(
              future: api.view(),
              builder: (context, snapshot) {
                if (snapshot.hasData) {
                  final viewModel = snapshot.data;
                  if (viewModel!.count == 0) {
                    return const Text(
                      "No Todo's in the List!",
                    );
                  } else {
                    return ListView.builder(
                      scrollDirection: Axis.vertical,
                      shrinkWrap: true,
                      itemCount: viewModel.count,
                      itemBuilder: (context, index) {
                        return ListTile(
                          title: Text(
                              '${index+1}. todo: ${viewModel.items[index]}'),
                        );
                      },
                    );

//                     return Text(
//                       '${viewModel.items}',
// //                    'From Rust: ${data?.items.first}',
//                     );
                  }
                } else if (snapshot.hasError) {
                  return Text('Error: ${snapshot.error}');
                } else {
                  return const CircularProgressIndicator();
                }
              },
            ),
            Column(
              children: [
                TextField(
                  controller: textController,
                  maxLines: null,
                ),
                ElevatedButton(
                  onPressed: () {
                    _addTodo(textController.text);
                  },
                  child: const Text("Add Todo"),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
