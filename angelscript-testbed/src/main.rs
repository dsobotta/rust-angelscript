

// engine
// AS_API asIScriptengine *asCreateScriptengine(asDWORD version);
// AS_API const char      *asGetLibraryVersion();
// AS_API const char      *asGetLibraryOptions();

pub mod angelscript {

    pub mod engine {

        use std::os::raw::c_char;
        use std::ffi::CStr;

        //pub struct Scriptengine {
            //asIScriptengine: *i32,
        //}

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

    } //end engine
} // end angelscript

fn main() {

    let as_version: String = angelscript::engine::get_library_version();
    println!("Angelscript Version: {}", as_version);

    let as_options: String = angelscript::engine::get_library_options();
    println!("Angelscript Library Options: {}", as_options);
}