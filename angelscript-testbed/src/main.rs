use std::os::raw::c_char;
use std::ffi::CStr;

extern { 
    #[link(name="angelscript_c", kind="static")]
    fn asGetLibraryVersion() -> *const c_char;
}

fn main() {
    let c_buf: *const c_char = unsafe { asGetLibraryVersion() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary

    println!("Angelscript Version: {}", str_buf);
}