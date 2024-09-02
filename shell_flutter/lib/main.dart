import 'package:flutter/material.dart';
import 'package:shell_flutter/state_handler.dart';

late final StateHandler stateHandler;
Future<void> main() async {
  stateHandler = await StateHandler.init();
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
            "-= Heavy item list simulator =-",
            style: Theme.of(context).textTheme.headlineMedium,
          ),
          _buildLastItem(),
          Row(
            children: [
              Expanded(
                child: TextField(
                  keyboardType: TextInputType.text,
                  controller: textController,
                  maxLines: null,
                  onSubmitted: (value) {
                    stateHandler.addItem(item: int.parse(value));
                    textController.clear();
                  },
                ),
              ),
              ElevatedButton(
                onPressed: () {
                  stateHandler.addItem(item: int.parse(textController.text));
                  textController.clear();
                },
                child: const Text("Add Item"),
              ),
            ],
          ),
          _buildItemList(),
        ],
      ),
    );
  }

  Widget _buildItemList() {
    // listens for the state stored in StateHandler
    return ValueListenableBuilder(
        valueListenable: stateHandler.items,
        builder: (context, items, _) {
          if (items.isEmpty) {
            return const Center(
              child: Text("No item in the List!"),
            );
          } else {
            return Expanded(
              child: ListView.builder(
                scrollDirection: Axis.vertical,
                shrinkWrap: true,
                itemCount: items.length,
                itemBuilder: (context, index) {
                  return ListTile(
                      title: Row(
                    children: [
                      ElevatedButton(
                        child: const Icon(Icons.remove),
                        onPressed: () {
                          stateHandler.removeItem(index: index);
                        },
                      ),
                      Text(' ${index + 1}.: ${items[index].heavy}'),
                    ],
                  ));
                },
              ),
            );
          }
        });
  }

  Widget _buildLastItem() {
    // listens for the state stored in StateHandler
    return ValueListenableBuilder(
        valueListenable: stateHandler.lastItem,
        builder: (context, item, _) {
          if (item == null) {
            return const Center(
              child: Text("THe List is empty!"),
            );
          } else {
            return Expanded(child: Text('Last Item: ${item.heavy}'));
          }
        });
  }
}
