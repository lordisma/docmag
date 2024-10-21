use std::sync::Arc;
use crate::common::error::*;

/// Writer trait for writing data to a persistent storage. 
/// This trait will result in an error type if the writing operation fails, 
/// must guarantee that volume of data remains unchanged, if not, it will 
/// panic.
pub trait Writer {
    /// Write operation to a persistent storage,
    /// 
    /// # Arguments
    /// `&data` - Reference to the data to be written to the storage.
    fn write(&mut self, data: Arc<&[u8]>) -> Result<(), WriteError>;
}