pub trait StringExt {
    fn mul(&self, times: i32) -> Self;
}

impl StringExt for String {
    fn mul(&self, times: i32) -> Self {
        let mut string_multed = self.clone();

        for _ in 1..times {
            string_multed.push_str(&self);
        }

        string_multed
    }
}


