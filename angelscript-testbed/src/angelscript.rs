
pub mod types;
pub mod engine;
mod engine_c;

use std::os::raw::c_char;
use std::ffi::CStr;

fn read_cstring(c_buf: *const c_char) -> String {

    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();

    return str_buf;
}

pub fn get_library_version() -> String {
    extern { 
        #[link(name="angelscript_c", kind="static")]
        fn asGetLibraryVersion() -> *const c_char;
    }

    let c_buf: *const c_char = unsafe { asGetLibraryVersion() };
    return read_cstring(c_buf);
}

pub fn get_library_options() -> String {
    extern { 
        #[link(name="angelscript_c", kind="static")]
        fn asGetLibraryOptions() -> *const c_char;
    }

    let c_buf: *const c_char = unsafe { asGetLibraryOptions() };
    return read_cstring(c_buf);
}