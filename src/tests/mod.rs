use crate::jb_inputs::input;

use super::*;
use std::io::ErrorKind;
//...

#[test]
fn make_err_pat1() {
   let msg = "Error Message";
   let err = io_err!(msg);
   assert_eq!(err.kind(), ErrorKind::Other);
   assert_eq!(err.to_string(), msg);
}
#[test]
fn make_err_pat2() {
   let msg = "Had a Already exists exception";
   let err = io_err!(ErrorKind::AlreadyExists, msg);
   assert_eq!(ErrorKind::AlreadyExists, err.kind());
   assert_eq!(msg, err.to_string());
}
