use std::os::fd::{self, AsRawFd};
use std::sync::Arc;
use crate::storage::writer::Writer;
use crate::common::error;
use libc;

/// File Writer
/// 
/// Represent a file where the content will be written, the aim of this is
/// to save the collection of documents as they come.
/// 
pub struct FileWriter {
    fd: fd::OwnedFd
}

impl Writer for FileWriter {
    fn write(&mut self, data: Arc<&[u8]>) -> Result<(), error::WriteError> {
        let bytes = data.as_ptr();
        let bytes_len = data.len();
        let result = unsafe {
            libc::write(self.fd.as_raw_fd(), bytes as *const libc::c_void, bytes_len)
        };

        match result {
            ..=-1 => {
                return Err(error::WriteError::TeaPot);
            },
            1.. => {
                return Ok(());
            },
            0 => {
                return Err(error::WriteError::TeaPot);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_write() {
        let file = File::create("/tmp/test_write").unwrap();
        let mut writer = FileWriter {
            fd: file.into()
        };
        let data = Arc::new("Hello, World!".as_bytes());
        let result = writer.write(data);
        assert!(result.is_ok());
    }
}