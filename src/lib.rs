//! A simple key/value store named **RiteKV**.
pub mod result;
pub mod storage;

pub use storage::{MemStore, Store};
