use crate::result::{KvsError, Result};
use crate::storage::{BatchStore, Store};

use sled::{Db, Tree};

use std::fmt::Display;

/// Wrapper of `sled::Db`
#[derive(Clone)]
pub struct SledStore(Db);

impl SledStore {
    /// Creates a `SledKvsEngine` from `sled::Db`.
    pub fn open(db: Db) -> Self {
        SledStore(db)
    }
}

impl Display for SledStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sledstore")
    }
}

impl Store for SledStore {
    #[inline]
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<Vec<u8>>> {
        let tree: &Tree = &self.0;
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        Ok(tree.get(key)?.map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec()))
    }

    #[inline]
    fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        let tree: &Tree = &self.0;
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        tree.insert(key, value.as_ref())?;
        tree.flush()?;
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        let tree: &Tree = &self.0;
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        tree.remove(key)?;
        tree.flush()?;
        Ok(())
    }

    #[inline]
    fn contains(&mut self, key: impl AsRef<[u8]>) -> Result<bool> {
        let tree: &Tree = &self.0;
        let key = key.as_ref().to_owned();
        if key.is_empty() {
            return Err(KvsError::EmptyKey);
        }
        Ok(tree.contains_key(key)?)
    }
}

impl BatchStore for SledStore {
    #[inline]
    fn get_batch(&self, keys: impl AsRef<[Vec<u8>]>) -> Result<Vec<Option<Vec<u8>>>> {
        let tree: &Tree = &self.0;
        let keys = keys.as_ref().to_owned();
        let values = keys
            .into_iter()
            .map(|key| tree.get(&key).ok()?.map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec()))
            .collect();
        Ok(values)
    }

    #[inline]
    fn set_batch(
        &mut self,
        keys: impl AsRef<[Vec<u8>]>,
        values: impl AsRef<[Vec<u8>]>,
    ) -> Result<()> {
        let tree: &Tree = &self.0;
        let keys = keys.as_ref().to_owned();
        let values = values.as_ref().to_owned();
        if keys.len() != values.len() {
            return Err(KvsError::InvalidData(
                "The number of keys does not match the number of values".to_string(),
            ));
        }
        for i in 0..keys.len() {
            let key = keys[i].to_vec();
            let value = values[i].to_vec();

            tree.insert(key, value)?;
        }
        Ok(())
    }

    #[inline]
    fn remove_batch(&mut self, keys: impl AsRef<[Vec<u8>]>) -> Result<()> {
        let tree: &Tree = &self.0;
        let keys = keys.as_ref().to_owned();
        for key in keys {
            tree.remove(&key)?;
        }
        Ok(())
    }
}
