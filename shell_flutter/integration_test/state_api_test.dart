import 'package:flutter_test/flutter_test.dart';
import 'package:shell_flutter/main.dart';
import 'package:shell_flutter/bridge/frb_generated/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await RustLib.init();
  });

  testWidgets('Can call rust function', (WidgetTester tester) async {
    // doesn't work, as file system is read only
    // final Directory appWriteableDir = await getApplicationSupportDirectory();
    // final stateFile = File('${appWriteableDir.path}/app_state.bin');
    await tester.pumpWidget(const MyApp());
    // await lifecycle.setup(path: stateFile.path);
    // Event add_todo_event = Event.addTodo("test todo!");
    // var result =
    //     await todo_list_api.processEvent(event: add_todo_event, hint: null);

    expect(find.textContaining('42'), findsOneWidget);
  });
}
