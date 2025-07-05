use std::{mem::ManuallyDrop, thread};

use async_trait::async_trait;
use irondash_message_channel::{
    AsyncMethodHandler, IntoValue, MethodCall, PlatformResult, TryFromValue,
};
use irondash_run_loop::RunLoop;
use log::debug;

pub struct Obj {}

#[derive(TryFromValue, IntoValue)]
pub struct ObjPayload {
    pub tag: String,
    pub value: String,
}

#[async_trait(?Send)]
impl AsyncMethodHandler for Obj {
    async fn on_method_call(&self, call: MethodCall) -> PlatformResult {
        match call.method.as_str() {
            "insert" => {
                println!(
                    "Received request {:?} on thread {:?}",
                    call,
                    std::thread::current().id()
                );
                let mut payload: ObjPayload = call.args.try_into()?;
                payload.value = "Inserted Value".to_string();
                Ok(payload.into())
            }
            _ => Err(irondash_message_channel::PlatformError {
                code: "invalid_method".into(),
                message: Some(format!("Unknown Method: {}", call.method)),
                detail: irondash_message_channel::Value::Null,
            }),
        }
    }
}

pub(crate) fn init() {
    // create Obj instance that will listen on main (platform) thread.
    let _ = ManuallyDrop::new(Obj {}.register("obj"));

    // create background thread and new Obj instance that will listen
    // on background thread (using different channel).
    thread::spawn(|| {
        let _ = ManuallyDrop::new(Obj {}.register("obj_background_thread"));
        debug!(
            "Running RunLoop on background thread {:?}",
            thread::current().id()
        );
        RunLoop::current().run();
    });
}
