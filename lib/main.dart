import 'package:flutter/material.dart';
import 'service.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Rust Message Channel',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      home: const MyHomePage(title: 'Flutter Rust Message Channel'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final MessageChannelService _service = MessageChannelService();
  final TextEditingController _messageController = TextEditingController();
  final TextEditingController _num1Controller = TextEditingController();
  final TextEditingController _num2Controller = TextEditingController();
  
  String _response = '';
  bool _isInitialized = false;

  @override
  void initState() {
    super.initState();
    _initializeService();
  }

  Future<void> _initializeService() async {
    try {
      await _service.initialize();
      setState(() {
        _isInitialized = true;
        _response = 'Service initialized successfully!';
      });
    } catch (e) {
      setState(() {
        _response = 'Failed to initialize service: $e';
      });
    }
  }

  Future<void> _sendMessage() async {
    if (!_isInitialized) {
      setState(() {
        _response = 'Service not initialized';
      });
      return;
    }

    final message = _messageController.text;
    if (message.isEmpty) {
      setState(() {
        _response = 'Please enter a message';
      });
      return;
    }

    final response = await _service.sendMessage(message);
    setState(() {
      _response = response ?? 'No response received';
    });
  }

  Future<void> _addNumbers() async {
    if (!_isInitialized) {
      setState(() {
        _response = 'Service not initialized';
      });
      return;
    }

    final num1 = int.tryParse(_num1Controller.text);
    final num2 = int.tryParse(_num2Controller.text);
    
    if (num1 == null || num2 == null) {
      setState(() {
        _response = 'Please enter valid numbers';
      });
      return;
    }

    final result = await _service.addNumbers(num1, num2);
    setState(() {
      _response = result != null ? 'Result: $result' : 'Failed to add numbers';
    });
  }

  Future<void> _getSystemInfo() async {
    if (!_isInitialized) {
      setState(() {
        _response = 'Service not initialized';
      });
      return;
    }

    final info = await _service.getSystemInfo();
    setState(() {
      _response = info != null ? 'System Info: ${info.join(', ')}' : 'Failed to get system info';
    });
  }

  @override
  void dispose() {
    _service.dispose();
    _messageController.dispose();
    _num1Controller.dispose();
    _num2Controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'Status: ${_isInitialized ? 'Connected' : 'Disconnected'}',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                color: _isInitialized ? Colors.green : Colors.red,
              ),
            ),
            const SizedBox(height: 20),
            
            TextField(
              controller: _messageController,
              decoration: const InputDecoration(
                labelText: 'Enter message',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              onPressed: _sendMessage,
              child: const Text('Send Message'),
            ),
            const SizedBox(height: 20),
            
            Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _num1Controller,
                    decoration: const InputDecoration(
                      labelText: 'Number 1',
                      border: OutlineInputBorder(),
                    ),
                    keyboardType: TextInputType.number,
                  ),
                ),
                const SizedBox(width: 10),
                Expanded(
                  child: TextField(
                    controller: _num2Controller,
                    decoration: const InputDecoration(
                      labelText: 'Number 2',
                      border: OutlineInputBorder(),
                    ),
                    keyboardType: TextInputType.number,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              onPressed: _addNumbers,
              child: const Text('Add Numbers'),
            ),
            const SizedBox(height: 20),
            
            ElevatedButton(
              onPressed: _getSystemInfo,
              child: const Text('Get System Info'),
            ),
            const SizedBox(height: 20),
            
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                border: Border.all(color: Colors.grey),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Text(
                'Response: $_response',
                style: const TextStyle(fontSize: 16),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
