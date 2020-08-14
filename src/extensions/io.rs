use crate::Error;
use std::io::Read;

pub trait EasyRead {
    fn e_read(&mut self, buf: &mut [u8], read: &mut usize) -> Result<usize, Error>;
}

impl<T> EasyRead for T
where
    T: Read,
{
    fn e_read(&mut self, buf: &mut [u8], read: &mut usize) -> Result<usize, Error> {
        let n = self
            .read(buf)
            .map_err(|e| Error::ReadError(e.to_string()))?;
        *read = n;
        Ok(n)
    }
}

