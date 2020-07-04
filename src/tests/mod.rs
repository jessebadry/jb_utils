
#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    //...
    #[test]
    fn make_err_pat1() {
       let err = crate::io_err!( "Error Message");
       println!("{:?}", err.kind());
    }
    #[test]
    fn make_err_pat2() {
       let err = crate::io_err!(ErrorKind::AlreadyExists, "Had a Already exists exception");
       println!("{:?}", err.kind());
    }
}
