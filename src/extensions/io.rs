use std::io::Read;
use std::io;
pub trait EasyRead {
    fn read_inplace(&mut self, len: usize) -> io::Result<Vec<u8>>;
    fn e_read(&mut self, buf: &mut [u8], read: &mut usize) -> io::Result<usize>;
}

impl<T> EasyRead for T
where
    T: Read,
{
    ///Reads the amount of bytes specified in the Reader.
    fn read_inplace(&mut self, len: usize) -> io::Result<Vec<u8>>{
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn e_read(&mut self, buf: &mut [u8], read: &mut usize) -> io::Result<usize> {
        *read = self.read(buf)?;
        Ok(*read)
    }
}
