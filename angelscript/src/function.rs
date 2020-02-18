use angelscript_sys::c_types::*;
use angelscript_sys::c_function::*;
//use crate::types::EReturnCodes;
//use crate::read_cstring;

//use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct ScriptFunction {
    pub(crate) function: *mut asIScriptFunction,
}

impl ScriptFunction {
    
    pub(crate) fn new(c_function: *mut asIScriptFunction) -> Option<ScriptFunction> {
        
        match c_function.is_null() {
            true => return None,
            false => return Some( ScriptFunction{ function: c_function } )
        }
    }
}