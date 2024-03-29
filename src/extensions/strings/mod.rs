mod chunks;
use chunks::Chunks;
pub trait StringExt {
    fn mul(&self, times: u32) -> String;
    fn count_char(&self, chr_bound: char) -> usize;
    fn per_slice(&self, chunks: usize) -> Chunks;
}
fn mul_impl<T: Into<String> + AsRef<str> + Copy>(string: T, times: u32) -> String {
    let mut string_multed = string.into();
    let string_ref = string.as_ref();
    for _ in 1..times {
        string_multed.push_str(string_ref);
    }

    string_multed
}
fn count_chars_impl(string: &str, chr_bound: char) -> usize {
    let mut i = 0;
    for chr in string.chars() {
        if chr_bound == chr {
            i += 1;
        }
    }
    i
}

impl StringExt for &str {
    fn mul(&self, times: u32) -> String {
        mul_impl(*self, times)
    }
    fn count_char(&self, chr_bound: char) -> usize {
        count_chars_impl(self, chr_bound)
    }
    fn per_slice(&self, chunks: usize) -> Chunks {
        Chunks::new(self, chunks)
    }
}

impl StringExt for String {
    fn mul(&self, times: u32) -> String {
        mul_impl(self, times)
    }
    fn count_char(&self, chr_bound: char) -> usize {
        count_chars_impl(self, chr_bound)
    }
    fn per_slice(&self, chunk_size: usize) -> Chunks {
        Chunks::new(self, chunk_size)
    }
}

#[test]
fn chunks_t() {
    let s = String::from("Hello").mul(1000); // size = 5 * 1000
    let slice_size = 100;
    assert_eq!(s.len() / slice_size, s.per_slice(slice_size).count());
}

//Performance test
#[test]
fn alternate_char_count_t() {
    fn count_chars_iter(string: &str, ch: char) -> usize {
        string.chars().filter(|chr| ch == *chr).count()
    }

    fn count_chars_for_loop(string: &str, ch: char) -> usize {
        let mut i = 0;
        for c in string.chars() {
            if c == ch {
                i += 1;
            }
        }
        i
    }
    use crate::debug::perf_timer::PerformanceTimer;
    let test = "test";
    let t1 = PerformanceTimer::new();
    count_chars_iter(test, 't');
    drop(t1);
    
    println!("Iteration Time ^");
    let t2 = PerformanceTimer::new();
    count_chars_for_loop(test, 't');
    drop(t2);
    println!("\nFor loop time ^ ");
}
