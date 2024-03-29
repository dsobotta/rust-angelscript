use angelscript_sys::c_types::*;
use angelscript_sys::c_context::*;

use crate::types::EReturnCodes;
use crate::types::EContextState;
use crate::function::ScriptFunction;
use crate::object::ScriptObject;
//use crate::read_cstring;

//use std::ffi::CString;
use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct ScriptContext {
    context: *mut asIScriptContext,
}

impl ScriptContext {
    
    pub(crate) fn new(c_context: *mut asIScriptContext) -> Option<ScriptContext> {
        
        match c_context.is_null() {
            true => return None,
            false => return Some( ScriptContext{ context: c_context } )
        }
    }

    pub fn prepare(&mut self, function: Option<ScriptFunction>) -> EReturnCodes {

        match function {
            None => return EReturnCodes::NoFunction,
            Some(func) => {
                let result = unsafe { asContext_Prepare(self.context, func.function) };
                return EReturnCodes::from_i32(result);
            }
        }
    }

    pub fn set_object(&mut self, object: &mut ScriptObject) -> EReturnCodes {

        let c_object: *mut c_void = object.obj as *mut _ as *mut c_void;
        let result = unsafe { asContext_SetObject(self.context, c_object) };
        return EReturnCodes::from_i32(result);
    }

    pub fn execute(&mut self) -> EContextState {

        let result = unsafe { asContext_Execute(self.context) };
        return EContextState::from_u32(result as u32);
    }

    pub fn get_return_dword(&mut self) -> u32 {
        
        let ret = unsafe { asContext_GetReturnDWord(self.context) };
        return ret as u32;
    }

    pub fn get_return_object(&mut self) -> Option<ScriptObject> {

        let ret = unsafe { asContext_GetAddressOfReturnValue(self.context) };
        let c_obj: *mut *mut asIScriptObject = ret as *mut *mut _ as *mut *mut asIScriptObject;

        let ptr = match c_obj.is_null() {
            true => std::ptr::null_mut(),
            false => unsafe { *c_obj }
        };

        return ScriptObject::new( ptr );
    }
}