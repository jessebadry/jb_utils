pub mod jb_inputs;
#[macro_use]
pub mod j_macs;
pub mod debug;
pub mod extensions;
pub mod j_fs;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    Other(String),
}
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type = match self {
            Error::Other(e) => ("Other", e),
        };
        let (name, content) = err_type;
        write!(f, "Type Of Error:'{}', Reason:'{}'", name, content)
    }
}
