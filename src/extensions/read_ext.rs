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

#[cfg(test)]
mod tests {
    use super::EasyRead;
    static test_file: &str = "test_text.txt";
    fn gen_test_text() {}
    #[test]
    fn read_test() {
        let mut file = std::fs::File::open(test_file).unwrap_or_else(|e| {
            std::fs::File::create(test_file)
                .unwrap_or_else(|e| panic!("Could not create test_text file for read_test. why: {}", e))
        });

        let mut r = 0;
        let buffer = &mut [0u8; 1000];
        while file.e_read(buffer, &mut r).unwrap_or(0) > 0 {
            println!("{:?}", &buffer[..r]);
        }
    }
}
