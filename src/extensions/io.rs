use crate::Error;
use std::io::Read;
pub trait EasyRead {
    fn read_inplace(&mut self, len: usize) -> Result<Vec<u8>, Error>;
    fn e_read(&mut self, buf: &mut [u8], read: &mut usize) -> Result<usize, Error>;
}

impl<T> EasyRead for T
where
    T: Read,
{
    fn read_inplace(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0u8; len];
        self.read(&mut buf)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(buf)
    }
    fn e_read(&mut self, buf: &mut [u8], read: &mut usize) -> Result<usize, Error> {
        let n = self
            .read(buf)
            .map_err(|e| Error::ReadError(e.to_string()))?;
        *read = n;
        Ok(n)
    }
}
