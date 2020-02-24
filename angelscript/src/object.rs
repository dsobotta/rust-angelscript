use angelscript_sys::c_types::*;
use angelscript_sys::c_object::*;
//use crate::types::EReturnCodes;
//use crate::read_cstring;

//use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct ScriptObject {
    pub(crate) obj: *mut asIScriptObject,
}

impl ScriptObject {
    
    pub(crate) fn new(c_object: *mut asIScriptObject) -> Option<ScriptObject> {
        
        match c_object.is_null() {
            true => return None,
            false => {
                unsafe{ asObject_AddRef(c_object) };
                return Some( ScriptObject{ obj: c_object } )
            }
        }
    }
}

impl Drop for ScriptObject {
    fn drop(&mut self) {
        unsafe { asObject_Release(self.obj) };
    }
}