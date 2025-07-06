// use std::{
//     ffi::c_char,
//     sync::{atomic::AtomicI64, RwLock},
// };

// use irondash_dart_ffi::{irondash_init_ffi, DartPort};

// use crate::ffi::CstrToRust;

// /// Globally accessible document directory (mutable).
// /// You can change it at runtime if needed.
// pub static DOCDIR: RwLock<String> = RwLock::new(String::new());

// /// Stores the Dart port for sending data back to Dart side.
// /// Wrapped in `RwLock` to allow both read and write access.
// static DART_PORT: AtomicI64 = AtomicI64::new(0);

// /// Returns a clone of the stored Dart port if available.
// pub fn get_dart_port() -> Option<DartPort> {
//     match DART_PORT.load(std::sync::atomic::Ordering::Relaxed) {
//         0 => None,
//         port => Some(DartPort::new(port)),
//     }
// }

// #[no_mangle]
// pub extern "C" fn set_reporting_port_ffi(port: i64) {
//     reporting_core::ports::set_reporting_port(port);
//     reporting_core::services::init_reporting_core();
// }

// #[no_mangle]
// pub extern "C" fn send_to_reporting_res_channel_ffi(message: *const c_char) {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         if let Err(err) = reporting_core::ports::REPORTING_RESPONSE_CHANNEL
//             .0
//             .send(message.to_native())
//             .await
//         {
//             log::error!("Failed to send message to reporting channel: {}", err);
//         }
//     });
// }

// /// Sets the Dart port from native code, typically called from Dart FFI layer.
// ///
// /// # Safety
// ///
// /// Exposed as an extern "C" function, so handle carefully.
// /// In your Dart FFI code, call it like:
// /// ```dart
// /// final setPort = nativeLib.lookupFunction<Void Function(Int64), void Function(int)>('set_dart_port');
// /// setPort(port);
// /// ```
// #[no_mangle]
// pub extern "C" fn set_dart_port_ffi(port: i64) {
//     set_dart_port(port);

//     // _ = std::thread::spawn(|| {
//     //     let large_string = {
//     //         let mut s = String::new();
//     //         for _ in 0..1000 {
//     //             s.push_str("Hello, World! ");
//     //         }
//     //         s
//     //     };
//     //     loop {
//     //         std::thread::sleep(std::time::Duration::from_millis(50));
//     //         send_string_to_dart(large_string.clone());
//     //     }
//     // });
// }

// pub fn set_dart_port(port: i64) {
//     println!("---- Setting Dart port to: {}", port);
//     DART_PORT.store(port, std::sync::atomic::Ordering::Relaxed);
//     println!("---- Dart port seted");
// }
// /// Initializes the Dart FFI runtime. Must be called before using FFI functions.
// /// Returns `true` on success.
// ///
// /// # Safety
// ///
// /// Exposed as an extern "C" function.
// /// Pass in `NativeApi.initializeApiDLData` from Dart:
// /// ```dart
// /// final initDartApi = nativeLib.lookupFunction<Uint8 Function(Pointer<Void>), int Function(Pointer<Void>)>('init_dart_api');
// /// final result = initDartApi(NativeApi.initializeApiDLData);
// /// ```
// #[no_mangle]
// pub extern "C" fn init_dart_api(data: *mut std::ffi::c_void) -> bool {
//     irondash_init_ffi(data);
//     true
// }

// /// Sends an integer to the Dart side.
// /// If the Dart port is not yet set, logs an error.
// pub fn send_i64_to_dart(value: i64) {
//     if let Some(dart_port) = get_dart_port() {
//         dart_port.send(value);
//     } else {
//         log::error!("Failed to send integer to Dart: Dart port not initialized!");
//     }
// }

// /// Sends an integer to the Dart side.
// /// If the Dart port is not yet set, logs an error.
// pub fn send_i32_to_dart(value: i32) {
//     if let Some(dart_port) = get_dart_port() {
//         dart_port.send(value);
//     } else {
//         log::error!("Failed to send integer to Dart: Dart port not initialized!");
//     }
// }

// /// Sends a string to the Dart side.
// /// If the Dart port is not yet set, logs an error.
// pub fn send_str_to_dart(message: &str) {
//     if let Some(dart_port) = get_dart_port() {
//         dart_port.send(message.to_string());
//     } else {
//         log::error!("Failed to send string to Dart: Dart port not initialized!");
//     }
// }

// /// Sends a string to the Dart side.
// /// If the Dart port is not yet set, logs an error.
// pub fn send_string_to_dart(message: String) {
//     if let Some(dart_port) = get_dart_port() {
//         dart_port.send(message.clone());
//     } else {
//         log::error!("Failed to send string to Dart: Dart port not initialized!");
//     }
// }

// /// Sends a i32 and a string as array to the Dart side.
// /// If the Dart port is not yet set, logs an error.
// pub fn send_i32_str_to_dart(i32_value: i32, str_value: &str) {
//     if let Some(dart_port) = get_dart_port() {
//         let data = vec![
//             irondash_dart_ffi::DartValue::from(i32_value),
//             irondash_dart_ffi::DartValue::from(str_value),
//         ];
//         dart_port.send(data);
//     } else {
//         log::error!("Failed to send array to Dart: Dart port not initialized!");
//     }
// }

// /// Sends a i32 and 2 strings as array to the Dart side.
// /// If the Dart port is not yet set, logs an error.
// pub fn send_i32_str2_to_dart(i32_value: i32, str1: &str, str2: &str) {
//     if let Some(dart_port) = get_dart_port() {
//         let data = vec![
//             irondash_dart_ffi::DartValue::from(i32_value),
//             irondash_dart_ffi::DartValue::from(str1),
//             irondash_dart_ffi::DartValue::from(str2),
//         ];
//         dart_port.send(data);
//     } else {
//         log::error!("Failed to send array to Dart: Dart port not initialized!");
//     }
// }

// /// Sends a Rdc enum value and a string value to Dart.
// /// This is primarily used for communication between Rust and Dart
// /// for the master WebSocket connection.
// pub fn send_to_dart_port(rdc_value: i32, str_value: &str) {
//     if let Some(dart_port) = get_dart_port() {
//         let data = vec![
//             irondash_dart_ffi::DartValue::from(rdc_value),
//             irondash_dart_ffi::DartValue::from(str_value),
//         ];
//         dart_port.send(data);
//     } else {
//         log::error!("Failed to send to Dart port: Dart port not initialized!");
//     }
// }
