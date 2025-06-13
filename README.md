# Flutter-Rust Message Channel Example

A complete implementation example of **irondash_message_channel** for high-performance communication between Flutter (Dart) and Rust.

## 🚀 Overview

This project demonstrates how to use `irondash_message_channel` to create a bridge between Flutter and Rust, enabling:
- ⚡ **High-performance** method calls
- 🔄 **Bidirectional communication**
- 📦 **Zero-copy data transfer**
- 🔒 **Type-safe** message passing

## 📁 Project Structure

```
flutter_rust_message_channe/
├── lib/
│   ├── main.dart              # Flutter UI and app logic
│   └── service.dart           # Message channel service layer
├── rust/
│   ├── src/
│   │   └── lib.rs            # Rust message handler implementation
│   └── Cargo.toml            # Rust dependencies
├── pubspec.yaml              # Flutter dependencies
└── README.md                 # This file
```

## 🛠️ Implementation

### 1. Flutter Side Implementation

#### Dependencies (`pubspec.yaml`)
```yaml
dependencies:
  flutter:
    sdk: flutter
  irondash_message_channel: ^0.7.0
```

#### Service Layer (`lib/service.dart`)
```dart
import 'dart:io';
import 'package:irondash_message_channel/irondash_message_channel.dart';

class MessageChannelService {
  static const String _channelName = 'flutter_rust_bridge';
  NativeMethodChannel? _channel;
  bool _isInitialized = false;

  Future<void> initialize() async {
    try {
      // Create message channel context pointing to Rust init function
      final context = MessageChannelContext.forInitFunction(initialize_rust_bridge);
      _channel = NativeMethodChannel(_channelName, context: context);
      _isInitialized = true;
      print('Message channel initialized successfully');
    } catch (e) {
      print('Message channel initialization failed: $e');
      rethrow;
    }
  }

  Future<String?> sendMessage(String message) async {
    if (!_isInitialized || _channel == null) {
      return 'Channel not initialized';
    }
    
    try {
      final response = await _channel!.invokeMethod('send_message', {
        'message': message
      });
      return response?.toString();
    } catch (e) {
      print('Error sending message: $e');
      return 'Error: ${e.toString()}';
    }
  }

  Future<int?> addNumbers(int a, int b) async {
    if (!_isInitialized || _channel == null) {
      return null;
    }
    
    try {
      final response = await _channel!.invokeMethod('add_numbers', {
        'a': a, 
        'b': b
      });
      return response as int?;
    } catch (e) {
      print('Error adding numbers: $e');
      return null;
    }
  }

  Future<List<String>?> getSystemInfo() async {
    if (!_isInitialized || _channel == null) {
      return null;
    }
    
    try {
      final response = await _channel!.invokeMethod('get_system_info', {});
      return (response as List?)?.cast<String>();
    } catch (e) {
      print('Error getting system info: $e');
      return null;
    }
  }

  void dispose() {
    _isInitialized = false;
  }
}
```

### 2. Rust Side Implementation

#### Dependencies (`rust/Cargo.toml`)
```toml
[package]
name = "flutter_rust_bridge"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
irondash_message_channel = "0.7.0"
```

#### Message Handler (`rust/src/lib.rs`)
```rust
use irondash_message_channel::*;
use std::sync::OnceLock;

struct MessageHandler;

impl MessageHandler {
    fn get_string_from_map(map: &ValueTupleList, key: &str) -> Option<String> {
        for (k, v) in map.iter() {
            if let Value::String(key_str) = k {
                if key_str == key {
                    if let Value::String(value) = v {
                        return Some(value.clone());
                    }
                }
            }
        }
        None
    }

    fn get_i64_from_map(map: &ValueTupleList, key: &str) -> Option<i64> {
        for (k, v) in map.iter() {
            if let Value::String(key_str) = k {
                if key_str == key {
                    if let Value::I64(value) = v {
                        return Some(*value);
                    }
                }
            }
        }
        None
    }
}

impl MethodHandler for MessageHandler {
    fn on_method_call(&self, call: MethodCall, reply: MethodCallReply) {
        let method = call.method.as_str();
        let args = &call.args;

        let result = match method {
            "send_message" => {
                if let Value::Map(map) = args {
                    if let Some(message) = Self::get_string_from_map(map, "message") {
                        let response = format!("Echo from Rust: {}", message);
                        Ok(Value::String(response))
                    } else {
                        Err(PlatformError {
                            code: "INVALID_ARGUMENT".to_string(),
                            message: Some("Message not found".to_string()),
                            detail: Value::Null,
                        })
                    }
                } else {
                    Err(PlatformError {
                        code: "INVALID_ARGUMENT".to_string(),
                        message: Some("Invalid arguments".to_string()),
                        detail: Value::Null,
                    })
                }
            }
            "add_numbers" => {
                if let Value::Map(map) = args {
                    let a = Self::get_i64_from_map(map, "a").unwrap_or(0);
                    let b = Self::get_i64_from_map(map, "b").unwrap_or(0);
                    let result = a + b;
                    Ok(Value::I64(result))
                } else {
                    Err(PlatformError {
                        code: "INVALID_ARGUMENT".to_string(),
                        message: Some("Invalid arguments".to_string()),
                        detail: Value::Null,
                    })
                }
            }
            "get_system_info" => {
                let info = vec![
                    format!("OS: {}", std::env::consts::OS),
                    format!("Architecture: {}", std::env::consts::ARCH),
                    format!("Family: {}", std::env::consts::FAMILY),
                    "Language: Rust".to_string(),
                ];
                let values: Vec<Value> = info.into_iter().map(Value::String).collect();
                Ok(Value::List(values))
            }
            _ => Err(PlatformError {
                code: "NOT_IMPLEMENTED".to_string(),
                message: Some(format!("Unknown method: {}", method)),
                detail: Value::Null,
            }),
        };

        reply.send(result);
    }
}

static HANDLER: OnceLock<MessageHandler> = OnceLock::new();

#[no_mangle]
pub extern "C" fn initialize_rust_bridge(context: *mut std::ffi::c_void) -> i32 {
    let handler = MessageHandler;
    if HANDLER.set(handler).is_err() {
        eprintln!("Failed to set handler - already initialized");
        return -1;
    }

    let _result = irondash_init_message_channel_context(context);
    
    println!("Rust message channel initialized successfully");
    0
}
```

## 🚀 Getting Started

### Prerequisites
- **Flutter SDK** (3.0.0+)
- **Rust** (1.70.0+)
- **Dart** (3.0.0+)

### Setup Instructions

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd flutter_rust_message_channe
   ```

2. **Install Flutter dependencies**
   ```bash
   flutter pub get
   ```

3. **Build the Rust library**
   ```bash
   cd rust
   cargo build --release
   cd ..
   ```

4. **Run the Flutter app**
   ```bash
   flutter run
   ```

## 🔧 How It Works

### Architecture Overview
```
┌─────────────────┐    irondash_message_channel    ┌─────────────────┐
│   Flutter App   │ ◄──────────────────────────► │   Rust Backend  │
│     (Dart)      │                               │                 │
│                 │    Method Calls & Responses   │                 │
│  - UI Layer     │                               │  - Message      │
│  - Service      │                               │    Handler      │
│  - Channel      │                               │  - Business     │
│                 │                               │    Logic        │
└─────────────────┘                               └─────────────────┘
```

### Message Flow

1. **Flutter → Rust**
   ```dart
   // Flutter calls Rust method
   final response = await channel.invokeMethod('send_message', {'message': 'Hello'});
   ```

2. **Rust Processing**
   ```rust
   // Rust receives and processes the call
   fn on_method_call(&self, call: MethodCall, reply: MethodCallReply) {
       match call.method.as_str() {
           "send_message" => {
               // Process message and send response
               reply.send(Ok(Value::String("Echo: Hello".to_string())));
           }
       }
   }
   ```

3. **Response → Flutter**
   ```dart
   // Flutter receives the response
   print(response); // "Echo: Hello"
   ```

## 📝 Supported Methods

This example implements three basic methods:

| Method | Parameters | Return Type | Description |
|--------|------------|-------------|-------------|
| `send_message` | `{message: String}` | `String` | Echoes the message with "Echo from Rust:" prefix |
| `add_numbers` | `{a: int, b: int}` | `int` | Returns the sum of two numbers |
| `get_system_info` | `{}` | `List<String>` | Returns OS, architecture, and platform info |

## 🧪 Testing

### Run Rust Tests
```bash
cd rust
cargo test
```

### Run Flutter Tests
```bash
flutter test
```

### Manual Testing
1. Launch the app: `flutter run`
2. Test each function through the UI:
   - **Send Message**: Type "Hello World" → Get "Echo from Rust: Hello World"
   - **Add Numbers**: Enter 15 + 27 → Get "Result: 42"
   - **System Info**: Click button → Get OS and architecture details

## 🔍 Key Implementation Details

### Thread Safety
- Uses `std::sync::OnceLock` for thread-safe handler registration
- Ensures single initialization of the message channel

### Error Handling
- Comprehensive error handling on both Flutter and Rust sides
- Proper error propagation with meaningful messages
- Fallback mechanisms for graceful degradation

### Performance Considerations
- Zero-copy data transfer where possible
- Efficient value extraction from maps
- Minimal allocations in hot paths

## 🚨 Common Issues & Solutions

### Issue: "Channel not initialized"
**Solution**: Ensure `initialize()` is called before using the service
```dart
await _service.initialize();
```

### Issue: Rust compilation errors
**Solution**: Check Rust version and dependencies
```bash
rustc --version  # Should be 1.70.0+
cargo update
```

### Issue: "Method not found" errors
**Solution**: Ensure method names match exactly between Flutter and Rust
```dart
// Flutter
await channel.invokeMethod('send_message', {...});
```
```rust
// Rust
match call.method.as_str() {
    "send_message" => { ... }  // Must match exactly
}
```

## 📚 Extensions & Customization

### Adding New Methods

1. **Add to Flutter service**
   ```dart
   Future<YourType?> yourMethod(YourParams params) async {
     final response = await _channel!.invokeMethod('your_method', params.toMap());
     return YourType.fromJson(response);
   }
   ```

2. **Add to Rust handler**
   ```rust
   "your_method" => {
       // Your implementation
       Ok(Value::from(your_result))
   }
   ```

### Advanced Data Types
- Support for complex nested structures
- Custom serialization/deserialization
- Binary data transfer

### Error Handling
- Custom error codes and messages
- Structured error responses
- Retry mechanisms

## 📖 References

- [irondash_message_channel Documentation](https://pub.dev/packages/irondash_message_channel)
- [Flutter Platform Channels](https://docs.flutter.dev/development/platform-integration/platform-channels)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ⭐ Support

If this example helped you, please give it a star! ⭐

For questions and support, please open an issue on GitHub.
