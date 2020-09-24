pub struct Chunks<'a> {
    iterations: usize,
    total_itered: usize,
    chunk_size: usize,
    data_len: usize,
    data: &'a str,
}
impl<'a> Chunks<'a> {
    pub fn new(data: &'a str, chunk_size: usize) -> Self {
        let data_len = data.len();
        let iterations = if data_len < chunk_size {
            0
        } else {
            data_len / chunk_size as usize
        };

        Chunks {
            iterations,
            total_itered: 0,
            chunk_size,
            data_len,
            data,
        }
    }
}
impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let start = self.total_itered;
        if start == self.data_len {
            return None;
        } else if self.iterations == 0 {
            let remaining = self.data_len % self.chunk_size;
            self.total_itered += remaining;
            return Some(&self.data[start..]);
        }
        let next = start + self.chunk_size;
        self.total_itered = next;
        Some(&self.data[start..next])
    }
}
