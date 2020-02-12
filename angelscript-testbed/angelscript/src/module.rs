use angelscript_sys::c_types::*;
// use angelscript_sys::c_module::*;
//use crate::read_cstring;

// use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct ScriptModule {
    module: *mut asIScriptModule,
}

impl ScriptModule {
    pub fn new(c_module: *mut asIScriptModule) -> Option<ScriptModule> {
        match c_module.is_null() {
            true => return None,
            false => return Some( ScriptModule{ module: c_module } )
        }
    }
}