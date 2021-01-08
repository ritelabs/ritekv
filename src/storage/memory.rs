use super::Store;

use crate::result::Result;

use std::fmt::Display;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

use griddle::HashMap;
use parking_lot::RwLock;
use seahash::SeaHasher;

/// The `MemStore` stores  key/value pairs.
///
/// In-memory key-value store using the `griddle` library `HashMap` implementation and not persisted to disk.
pub struct MemStore {
    storage: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>, BuildHasherDefault<SeaHasher>>>>,
}

impl MemStore {
    /// Creates a new Memory key-value storage engine.
    pub fn open() -> Self {
        MemStore { storage: Arc::new(RwLock::new(HashMap::default())) }
    }
}

impl Default for MemStore {
    fn default() -> Self {
        Self::open()
    }
}

impl Display for MemStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "memstore")
    }
}

impl Store for MemStore {
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<Vec<u8>>> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let storage = storage.read();
        Ok(storage.get(&key).cloned())
    }

    fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let value = value.as_ref().to_owned();
        let mut storage = storage.write();
        storage.insert(key, value);
        Ok(())
    }

    fn remove(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let mut storage = storage.write();
        storage.remove(&key);
        Ok(())
    }

    fn contains(&mut self, key: impl AsRef<[u8]>) -> Result<bool> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let storage = storage.read();
        Ok(storage.contains_key(&key))
    }
}

#[cfg(test)]
impl super::TestSuite<MemStore> for MemStore {
    fn setup() -> Result<Self> {
        Ok(MemStore::open())
    }
}

#[test]
fn tests() -> Result<()> {
    use super::TestSuite;
    MemStore::test()
}
