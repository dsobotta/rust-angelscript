use angelscript_sys::c_types::*;
use angelscript_sys::c_context::*;
use crate::types::EReturnCodes;
//use crate::read_cstring;

use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct ScriptContext {
    context: *mut asIScriptContext,
}

impl ScriptContext {
    
    pub fn new(c_context: *mut asIScriptContext) -> Option<ScriptContext> {
        
        match c_context.is_null() {
            true => return None,
            false => return Some( ScriptContext{ context: c_context } )
        }
    }

}