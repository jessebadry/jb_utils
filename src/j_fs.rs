use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
///Reads file from given offsets.
///
pub fn slice_read(fname: &str, offset: u64, to: u64) -> io::Result<Vec<u8>> {
    let mut file = File::open(fname)?;
    let mut buf = vec![0; (to - offset) as usize];

    if offset > 0 {
        file.seek(SeekFrom::Start(offset))?;
    }
    let bytes_read = file.read(&mut buf)?;
    Ok(buf[..bytes_read].to_vec())
}

#[test]
fn slice_read_t() {
    let t_file = "tests/slice_read.txt";
    std::fs::write(t_file, "Hello!").unwrap();

    assert_eq!(std::str::from_utf8(&slice_read(t_file, 2, 4).unwrap()).unwrap(), "ll");
}
