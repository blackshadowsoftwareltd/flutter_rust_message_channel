use irondash_message_channel::*;
use std::sync::OnceLock;

struct MessageHandler;

// Simple helper functions
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

// Use OnceLock instead of Late for thread safety
static HANDLER: OnceLock<MessageHandler> = OnceLock::new();

#[no_mangle]
pub extern "C" fn initialize_rust_bridge(context: *mut std::ffi::c_void) -> i32 {
    // Initialize handler
    let handler = MessageHandler;
    if HANDLER.set(handler).is_err() {
        eprintln!("Failed to set handler - already initialized");
        return -1;
    }

    // Initialize the message channel context
    let _result = irondash_init_message_channel_context(context);
    
    println!("Rust message channel initialized successfully");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_extraction() {
        let mut vec = Vec::new();
        vec.push((Value::String("message".to_string()), Value::String("Hello".to_string())));
        let map = ValueTupleList::new(vec);
        
        let result = MessageHandler::get_string_from_map(&map, "message");
        assert_eq!(result, Some("Hello".to_string()));
    }

    #[test]
    fn test_i64_extraction() {
        let mut vec = Vec::new();
        vec.push((Value::String("number".to_string()), Value::I64(42)));
        let map = ValueTupleList::new(vec);
        
        let result = MessageHandler::get_i64_from_map(&map, "number");
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_missing_key() {
        let vec = Vec::new();
        let map = ValueTupleList::new(vec);
        
        let result = MessageHandler::get_string_from_map(&map, "missing");
        assert_eq!(result, None);
    }
}