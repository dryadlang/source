// crates/dryad_runtime/src/lib.rs
pub mod interpreter;
pub mod native_modules;
pub mod errors;
pub mod resolver;
pub mod heap;
pub mod value;
pub mod debug;
pub mod debug_server;
pub mod environment;
pub mod native_registry;

pub use interpreter::{Interpreter, Value};
pub use native_modules::NativeModuleManager;
