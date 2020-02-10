use crate::angelscript::types::*;
use crate::angelscript::engine_c::*;
//use crate::angelscript::read_cstring;

use std::ffi::CString;
//use std::os::raw::c_char;
use std::os::raw::c_int;

pub struct ScriptEngine {
    engine: *mut asIScriptEngine
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        return ScriptEngine {
            engine: unsafe { asCreateScriptEngine(ANGELSCRIPT_VERSION) }
        }
    }

    pub fn get_global_function_count(&self) -> u32 {
        let count = unsafe { asEngine_GetGlobalFunctionCount(self.engine) };
        return count as u32;
    }

    pub fn set_message_callback(&mut self, callback: &asFUNCTION_t) {
        let nullptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let _result = unsafe { asEngine_SetMessageCallback(self.engine, *callback, nullptr, asECallConvTypes_asCALL_CDECL) };

    }

    /*
    pub struct asSMessageInfo {
        pub section: *const ::std::os::raw::c_char,
        pub row: ::std::os::raw::c_int,
        pub col: ::std::os::raw::c_int,
        pub type_: asEMsgType,
        pub message: *const ::std::os::raw::c_char,ScriptEngine
    }
    */
    pub fn send_message(&mut self, section: &str, row: u32, col: u32, msg_type: asEMsgType, message: &str) {
        let c_section = CString::new(section).expect("CString::new failed");
        let c_message = CString::new(message).expect("CString::new failed");
        let _result = unsafe { asEngine_WriteMessage(self.engine, c_section.as_ptr(), row as c_int, col as c_int, msg_type, c_message.as_ptr()) };

        //TODO: Do something with the result.
    }

}