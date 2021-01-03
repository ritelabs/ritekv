use super::Store;

use crate::result::Result;

use std::fmt::Display;

use griddle::HashMap;

/// The `MemStore` stores  key/value pairs.
///
/// In-memory key-value store using the `griddle` library `HashMap` implementation and not persisted to disk.
pub struct MemStore {
    storage: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemStore {
    /// Creates a new Memory key-value storage engine.
    pub fn open() -> Self {
        Self { storage: HashMap::new() }
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
        let key = key.as_ref().to_owned();
        Ok(self.storage.get(&key).cloned())
    }

    fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        let key = key.as_ref().to_owned();
        let value = value.as_ref().to_owned();
        self.storage.insert(key, value);
        Ok(())
    }

    fn remove(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        let key = key.as_ref().to_owned();
        self.storage.remove(&key);
        Ok(())
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
