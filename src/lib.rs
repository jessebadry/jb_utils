pub mod jb_inputs;
#[macro_use]
pub mod j_macs;
pub mod extensions;
pub mod str_extensions;
pub mod debug;

pub enum Error{
    ReadError(String),
}