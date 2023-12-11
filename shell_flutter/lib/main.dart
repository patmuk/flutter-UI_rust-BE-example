import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import 'package:shell_flutter/messages/crux.pbserver.dart';
import 'package:shell_flutter/messages/todo_list.pbserver.dart';

import 'messages/todo_list.pb.dart' as todo_list;
import 'messages/crux.pb.dart' as crux;

void main() async {
  await Rinf.ensureInitialized();
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
  var viewModel = todo_list.ViewModel(); 

  // this triggers the button event in the rust lib
  Future<void> _processEvent(crux.Event event) async {
    final processEventRequest = crux.ProcessEvent(event: event);
    final rustRequest = RustRequest(
      resource: crux.ID, 
      operation: RustOperation.Read, 
      message: processEventRequest.writeToBuffer());
    // send the request
    final rustResponse = await requestToRust(rustRequest);
    List<crux.Effect> effects;
    if(rustResponse.successful == true){
      effects = crux.HandleEffect.fromBuffer(rustResponse.message!).effects;
    } else {
      effects = [Effect(rustCallError: RustCallError())];
    }
//    var effects = await api.processEvent(event: event);
    _handleEffect(effects);
  }

  // this handles the Render Effect
  Future<void> _handleEffect(List<crux.Effect> effects) async {
    for (crux.Effect effect in effects) {
      // effect.when(render: (ViewModel viewModel) => setState(() {}));
      if (effect.whichEffect() == Effect_Effect.render){
        _getViewModel();
      // (render: (ViewModel viewModel) => setState(() {}));
      } 
    }
  }

  // this triggers the button event in the rust lib
  Future<ViewModel> _getViewModel() async {
    final getViewModel = crux.Effect(render: Render());
    final rustRequest = RustRequest(
      resource: crux.ID, 
      operation: RustOperation.Read, 
      message:  getViewModel.writeToBuffer());
    // send the request
    final rustResponse = await requestToRust(rustRequest);
    if(rustResponse.successful == true){
      setState(() {
        viewModel = todo_list.ViewModel.fromBuffer(rustResponse.message!);
      });
    }
    return viewModel;
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
                    _processEvent(Event(addTodo: AddTodoEvent(todo: value)));
                    textController.clear();
                  },
                ),
              ),
              ElevatedButton(
                onPressed: () {
                  _processEvent(Event(addTodo: AddTodoEvent(todo: textController.text)));
                  textController.clear();
                },
                child: const Text("Add Todo"),
              ),
            ],
          ),
          // use 'setState' for triggering a refresh, which calling the future
          FutureBuilder(
            future: _getViewModel(),
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
                                _processEvent(Event(removeTodo: RemoveTodoEvent(index: index + 1)));
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
