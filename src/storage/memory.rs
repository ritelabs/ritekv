use crate::result::{KvsError, Result};
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

use serde::{Deserialize, Serialize};

type SeaHashMap = HashMap<Vec<u8>, Vec<u8>, BuildHasherDefault<SeaHasher>>;

/// The `MemStore` stores  key/value pairs.
///
/// In-memory key-value store using `HashMap` implementation and not persisted to disk.
#[derive(Serialize, Deserialize, Debug)]
pub struct MemStore {
    #[serde(with = "arc_rwlock_serde")]
    storage: Arc<RwLock<SeaHashMap>>,
}

impl MemStore {
    /// Creates a new Memory key-value storage engine.
    #[inline]
    pub fn open() -> Self {
        MemStore { storage: Arc::new(RwLock::new(SeaHashMap::default())) }
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
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        let storage = storage.read();
        Ok(storage.get(&key).cloned())
    }

    #[inline]
    fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        let value = value.as_ref().to_owned();
        let mut storage = storage.write();
        storage.insert(key, value);
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        let mut storage = storage.write();
        storage.remove(&key);
        Ok(())
    }

    #[inline]
    fn contains(&mut self, key: impl AsRef<[u8]>) -> Result<bool> {
        let storage = Arc::clone(&self.storage);
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
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
        if keys.len() != values.len() {
            return Err(KvsError::InvalidData(
                "The number of keys does not match the number of values".to_string(),
            ));
        }
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

#[test]
fn test_empty_key_error() {
    let mut store = MemStore::open();

    let key = b"".to_vec();

    match store.set(key, vec![0x01]) {
        Err(KvsError::EmptyKey) => (), // pass
        _ => panic!("should return error KvsError::EmptyKey"),
    }
}

#[test]
fn test_invalid_data_error() {
    let mut store = MemStore::open();

    let keys = b"".to_vec();

    match store.set_batch(vec![keys], vec![]) {
        Err(KvsError::InvalidData(_)) => (), // pass
        _ => panic!("should return error KvsError::InvalidData"),
    }
}

mod arc_rwlock_serde {
    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    use parking_lot::RwLock;

    pub fn serialize<S, T>(val: &Arc<RwLock<T>>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        T::serialize(&*val.read(), s)
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Ok(Arc::new(RwLock::new(T::deserialize(d)?)))
    }
}
