#[macro_export]
macro_rules! io_err {
    ($msg:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $msg)
    };
    ($err_kind:expr, $msg:expr) => {
        std::io::Error::new($err_kind, $msg)
    };
}
