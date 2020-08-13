extern crate libc;
use self::libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;


pub trait EasyCString {
    fn as_str(&self) -> String;
}
pub trait EasyFFiString {
    fn into_ffi(&self) -> *mut c_char;
}
impl EasyFFiString for String {
    fn into_ffi(&self) -> *mut c_char {
        CString::new(self.as_bytes()).unwrap().into_raw()
    }
}
impl EasyCString for *const c_char {
    fn as_str(&self) -> String {
        let s1 = unsafe { CStr::from_ptr(*self) };
        s1.to_str().unwrap().into()
    }
}