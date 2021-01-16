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

#[cfg(test)]
trait TestBatchSuite<B: Store + BatchStore> {
    fn setup() -> Result<B>;

    fn test() -> Result<()> {
        Self::test_get_batch()?;
        Self::test_set_batch()?;
        Self::test_remove_batch()?;
        Ok(())
    }

    fn test_get_batch() -> Result<()> {
        let mut s = Self::setup()?;
        let data1 = b"test1".to_vec();
        let data2 = b"test2".to_vec();
        s.set(data1.clone(), data1.clone()).unwrap();
        s.set(data2.clone(), data2.clone()).unwrap();
        assert_eq!(s.get(data1.clone())?, Some(data1.clone()));
        assert_eq!(s.get(data2.clone())?, Some(data2.clone()));
        assert_eq!(
            s.get_batch(vec![data1.clone(), data2.clone()])?,
            vec![Some(data1), Some(data2)]
        );
        Ok(())
    }

    fn test_set_batch() -> Result<()> {
        let mut s = Self::setup()?;
        let data1 = b"test1".to_vec();
        let data2 = b"test2".to_vec();
        s.set_batch(vec![data1.clone(), data2.clone()], vec![data1.clone(), data2.clone()])
            .unwrap();
        assert_eq!(s.get(data1.clone())?, Some(data1.clone()));
        assert_eq!(s.get(data2.clone())?, Some(data2.clone()));
        Ok(())
    }

    fn test_remove_batch() -> Result<()> {
        let mut s = Self::setup()?;
        let data1 = b"test1".to_vec();
        let data2 = b"test2".to_vec();
        s.set(data1.clone(), data1.clone()).unwrap();
        s.set(data2.clone(), data2.clone()).unwrap();
        assert_eq!(s.get(data1.clone())?, Some(data1.clone()));
        assert_eq!(s.get(data2.clone())?, Some(data2.clone()));
        s.remove_batch(&[data1.clone(), data2.clone()]).unwrap();
        assert_eq!(s.get(data1)?, None);
        assert_eq!(s.get(data2)?, None);
        Ok(())
    }
}
