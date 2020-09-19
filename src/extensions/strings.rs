pub trait StringExt {
    fn mul(&self, times: u32) -> String;
    fn count_char(&self, chr_bound: char) -> usize;
}
fn mul_impl<T: Into<String>>(string: T, times: u32) -> String {
    let mut string_multed = string.into();
    let copy = string_multed.clone();
    for _ in 1..times {
        string_multed.push_str(&copy);
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
}

impl StringExt for String {
    fn mul(&self, times: u32) -> String {
        mul_impl(self, times)
    }
    fn count_char(&self, chr_bound: char) -> usize {
        count_chars_impl(self, chr_bound)
    }
}
