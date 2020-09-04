use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::io;
pub fn buffered_read(fname: &str, offset: u64, to: u64) -> io::Result<Vec<u8>> {
    let mut file = File::open(fname)?;
    let mut buf = vec![0; (to - offset) as usize];

    if offset > 0 {
        file.seek(SeekFrom::Start(offset))?;
    }
    let bytes_read = file.read(&mut buf)?;
    Ok(buf[..bytes_read].to_vec())
}
