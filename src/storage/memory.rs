use crate::result::Result;
use crate::storage::{BatchStore, Store};

#[cfg(not(feature = "amortized"))]
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

#[cfg(feature = "amortized")]
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
    #[inline]
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
    #[inline]
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<Vec<u8>>> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let storage = storage.read();
        Ok(storage.get(&key).cloned())
    }

    #[inline]
    fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let value = value.as_ref().to_owned();
        let mut storage = storage.write();
        storage.insert(key, value);
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let mut storage = storage.write();
        storage.remove(&key);
        Ok(())
    }

    #[inline]
    fn contains(&mut self, key: impl AsRef<[u8]>) -> Result<bool> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        let storage = storage.read();
        Ok(storage.contains_key(&key))
    }
}

impl BatchStore for MemStore {
    #[inline]
    fn get_batch(&self, keys: impl AsRef<[Vec<u8>]>) -> Result<Vec<Option<Vec<u8>>>> {
        let storage = Arc::clone(&self.storage);
        let keys = keys.as_ref().to_owned();
        let storage = storage.read();
        let values = keys.into_iter().map(|key| storage.get(&key).map(|v| v.to_vec())).collect();
        Ok(values)
    }

    #[inline]
    fn set_batch(
        &mut self,
        keys: impl AsRef<[Vec<u8>]>,
        values: impl AsRef<[Vec<u8>]>,
    ) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let keys = keys.as_ref().to_owned();
        let values = values.as_ref().to_owned();
        let mut storage = storage.write();
        for i in 0..keys.len() {
            let key = keys[i].to_vec();
            let value = values[i].to_vec();

            storage.insert(key, value);
        }
        Ok(())
    }

    #[inline]
    fn remove_batch(&mut self, keys: impl AsRef<[Vec<u8>]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let keys = keys.as_ref().to_owned();
        let mut storage = storage.write();
        for key in keys {
            storage.remove(&key);
        }
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
fn test_basic() -> Result<()> {
    use super::TestSuite;
    MemStore::test()
}

#[cfg(test)]
impl super::TestBatchSuite<MemStore> for MemStore {
    fn setup() -> Result<Self> {
        Ok(MemStore::open())
    }
}

#[test]
fn test_batch() -> Result<()> {
    use super::TestBatchSuite;
    MemStore::test()
}
