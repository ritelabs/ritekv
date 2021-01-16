//! A simple key/value store named **RiteKV**.
//!
//! **Work in progress, you can now think of it as a simple in-memory key-value store and evaluate its API.**
//!
//! ## Examples
//!
//! ```
//! use ritekv::{MemStore, Store};
//! # fn main() {
//! let mut store = MemStore::open();
//! store.set("beep", "boop").unwrap();
//! let value = store.get("beep").unwrap();
//! assert_eq!(value, Some("boop".as_bytes().to_owned()));
//! # }
//! ```

pub mod result;
pub mod storage;

pub use storage::{BatchStore, MemStore, Store};
