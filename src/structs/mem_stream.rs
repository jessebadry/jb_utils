use std::io::{self, Read, Seek, SeekFrom, Write};

#[derive(Debug, Default)]
pub struct MemoryStream {
    data: Vec<u8>,
    position: usize,
}
impl MemoryStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            position: 0,
            data,
        }
    }
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
impl Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data_left = self.data.len() - self.position;

        if data_left == 0 {
            return Ok(0);
        }

        let buf_len = buf.len();

        let read = if data_left < buf_len {
            data_left
        } else {
            buf_len
        };

        buf[..read].copy_from_slice(&self.data[self.position..self.position + read]);

        self.position += read;

        Ok(read)
    }
}

impl Write for MemoryStream {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.data.extend_from_slice(buf);
        Ok(())
    }
}
impl Seek for MemoryStream {
    // TODO: Add other seek options
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let added_pos = match pos {
            SeekFrom::Start(pos) => {
                let remaining_bytes = (self.data.len() - self.position) as u64;
                if pos > remaining_bytes {
                    let new_pos = remaining_bytes % pos;
                    self.position += new_pos as usize;

                    new_pos
                } else {
                    self.position = pos as usize;

                    pos
                }
            }

            _ => unimplemented!(),
        };

        Ok(added_pos)
    }
}

mod tests {
    use super::*;


    #[test]
    fn test_mem_seek() {
        let mut mem_stream = MemoryStream::new([1, 2, 3].to_vec());
        
        assert_eq!(3, mem_stream.seek(SeekFrom::Start(10)).unwrap());
        assert_eq!(0, mem_stream.seek(SeekFrom::Start(0)).unwrap());
        assert_eq!(mem_stream.position, 0);

    }
}
