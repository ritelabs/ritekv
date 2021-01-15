mod memory;

pub use memory::MemStore;

use crate::result::Result;

use std::fmt::Display;

/// A key/value store trait for basic ops.
pub trait Store: Display + Send + Sync {
    /// Gets a value for a key, if it exists.
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<Vec<u8>>>;

    /// Sets a value for a key, replacing the existing value if any.
    fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()>;

    /// Removes a key, or does nothing if it does not exist.
    fn remove(&mut self, key: impl AsRef<[u8]>) -> Result<()>;

    // Returns `true` if the store contains a value for the specified key.
    fn contains(&mut self, key: impl AsRef<[u8]>) -> Result<bool>;
}

/// A key/value store trait for batch ops.
pub trait BatchStore: Display + Send + Sync {
    /// Gets values for keys, if them exist.
    fn get_batch(&self, keys: impl AsRef<[Vec<u8>]>) -> Result<Vec<Option<Vec<u8>>>>;

    /// Sets values for keys, replacing the existing values if any.
    fn set_batch(
        &mut self,
        keys: impl AsRef<[Vec<u8>]>,
        values: impl AsRef<[Vec<u8>]>,
    ) -> Result<()>;

    /// Removes keys, or does nothing if them do not exist.
    fn remove_batch(&mut self, keys: impl AsRef<[Vec<u8>]>) -> Result<()>;
}

#[cfg(test)]
trait TestSuite<S: Store> {
    fn setup() -> Result<S>;

    fn test() -> Result<()> {
        Self::test_remove()?;
        Self::test_get()?;
        Self::test_set()?;
        Self::test_contains()?;
        Ok(())
    }

    fn test_get() -> Result<()> {
        let mut s = Self::setup()?;
        s.set(b"a", vec![0x01])?;
        assert_eq!(Some(vec![0x01]), s.get(b"a")?);
        assert_eq!(None, s.get(b"b")?);
        Ok(())
    }

    fn test_remove() -> Result<()> {
        let mut s = Self::setup()?;
        s.set(b"a", vec![0x01])?;
        assert_eq!(Some(vec![0x01]), s.get(b"a")?);
        s.remove(b"a")?;
        assert_eq!(None, s.get(b"a")?);
        s.remove(b"b")?;
        Ok(())
    }

    fn test_set() -> Result<()> {
        let mut s = Self::setup()?;
        s.set(b"a", vec![0x01])?;
        assert_eq!(Some(vec![0x01]), s.get(b"a")?);
        s.set(b"a", vec![0x02])?;
        assert_eq!(Some(vec![0x02]), s.get(b"a")?);
        Ok(())
    }

    fn test_contains() -> Result<()> {
        let mut s = Self::setup()?;
        s.set(b"a", vec![0x01])?;
        assert_eq!(true, s.contains(b"a")?);
        assert_eq!(false, s.contains(b"b")?);
        Ok(())
    }
}
