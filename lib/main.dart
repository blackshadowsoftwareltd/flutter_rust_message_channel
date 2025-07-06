import 'dart:convert';
import 'dart:ffi';
import 'dart:isolate';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:irondash_message_channel/irondash_message_channel.dart';

late final ReceivePort port;
final _dylib = defaultTargetPlatform == TargetPlatform.android
    ? DynamicLibrary.open("librust_ex.so")
    : (defaultTargetPlatform == TargetPlatform.windows ? DynamicLibrary.open("rust_ex.dll") : DynamicLibrary.process());

/// initialize context for Native library.
MessageChannelContext _initNativeContext() {
  // This function will be called by MessageChannel with opaque FFI
  // initialization data. From it you should call
  // `irondash_init_message_channel_context` and do any other initialization,
  // i.e. register rust method channel handlers.
  final function = _dylib.lookup<NativeFunction<MessageChannelContextInitFunction>>(
    "rust_ex_init_message_channel_context",
  );
  return MessageChannelContext.forInitFunction(function);
}

// Initializes the native code (registers the method channel handlers, etc).
// The initialization is done on platform thread. So native code will post
// a message on the port when it's done.
Future<void> _initNative() async {
  port = ReceivePort();
  final function = _dylib
      .lookup<NativeFunction<Void Function(Pointer<Void>, Int64)>>("rust_ex_init_native")
      .asFunction<void Function(Pointer<Void>, int)>();
  function(NativeApi.initializeApiDLData, port.sendPort.nativePort);
  port.listen((message) {
    print('Dart Receiveport Message: $message');
  });
  // return await port.first;
}

final nativeContext = _initNativeContext();

final _channel = NativeMethodChannel('addition_channel', context: nativeContext);
final _obj = NativeMethodChannel('obj', context: nativeContext);

final _channelBackgroundThread = NativeMethodChannel('addition_channel_background_thread', context: nativeContext);

final _slowChannel = NativeMethodChannel('slow_channel', context: nativeContext);

final _httpClientChannel = NativeMethodChannel('http_client_channel', context: nativeContext);

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  Future<void> _showResult(Object res) async {
    const encoder = JsonEncoder.withIndent('  ');
    final text = encoder.convert(res);
    await showDialog(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: const Text('Received from Rust'),
          content: Text(text),
          actions: <Widget>[
            TextButton(
              child: const Text('Continue'),
              onPressed: () {
                Navigator.of(context).pop();
              },
            ),
          ],
        );
      },
    );
  }

  void _passObj() async {
    final payload = ObjPayload(tag: 'order', value: '1234');
    final res = await _obj.invokeMethod('insert', payload.toJson());
    print(res);
  }

  void _listenObj() async {
    final payload = ObjPayload(tag: 'order', value: '1234');
    final res = await _obj.invokeMethod('listen', payload.toJson());
    print(res);
  }

  void _callRustOnPlatformThread() async {
    final res = await _channel.invokeMethod('add', {'a': 10.0, 'b': 20.0});
    await _showResult(res);
  }

  void _callRustOnBackgroundThread() async {
    final res = await _channelBackgroundThread.invokeMethod('add', {'a': 15.0, 'b': 5.0});
    await _showResult(res);
  }

  void _callSlowMethod() async {
    final res = await _slowChannel.invokeMethod('getMeaningOfUniverse', {});
    await _showResult(res);
  }

  void _loadPage() async {
    final res = await _httpClientChannel.invokeMethod('load', {'url': 'https://flutter.dev'});
    await _showResult(res);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            TextButton(onPressed: _callRustOnPlatformThread, child: const Text('Call Rust (main/platform thread)')),
            TextButton(onPressed: _callRustOnBackgroundThread, child: const Text('Call Rust (background thread)')),
            TextButton(onPressed: _callSlowMethod, child: const Text('Call Rust (slow method)')),
            TextButton(onPressed: _loadPage, child: const Text('Load page using Reqwest/Tokio')),
          ],
        ),
      ),
      floatingActionButton: ElevatedButton(onPressed: _passObj, onLongPress: _listenObj, child: const Icon(Icons.add)),
    );
  }
}

void main() async {
  await _initNative();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(primarySwatch: Colors.blue),
      home: const MyHomePage(),
    );
  }
}

class ObjPayload {
  final String tag;
  final String value;
  ObjPayload({required this.tag, required this.value});
  Map<String, dynamic> toJson() {
    final result = <String, dynamic>{};
    result.addAll({'tag': tag});
    result.addAll({'value': value});
    return result;
  }

  factory ObjPayload.fromJson(Map<String, dynamic> map) => ObjPayload(tag: map['tag'] ?? '', value: map['value'] ?? '');
  String toRawJson() => json.encode(toJson());
  factory ObjPayload.fromRawJson(String source) => ObjPayload.fromJson(json.decode(source));
}
