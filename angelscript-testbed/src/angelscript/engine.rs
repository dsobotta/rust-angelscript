use crate::angelscript::types::*;
use crate::angelscript::engine_c::*;
use crate::angelscript::read_cstring;

use std::ffi::CString;
use std::ffi::c_void;
//use std::os::raw::c_char;
use std::os::raw::c_int;

pub struct MessageInfo {
    pub section: String,
    pub row: u32,
    pub col: u32,
    pub msg_type: asEMsgType,
    pub message: String
}

pub type ASMessageCallbackFunc = fn(MessageInfo);

pub struct ScriptEngine {
    engine: *mut asIScriptEngine,
    msg_callback: Option<ASMessageCallbackFunc>
}

impl ScriptEngine {

    pub fn new() -> ScriptEngine {
        return ScriptEngine {
            engine: unsafe { asCreateScriptEngine(ANGELSCRIPT_VERSION) },
            msg_callback: None
        }
    }

    unsafe extern "C" fn cvoid_msg_callback(msg_ptr: *const asSMessageInfo, params: *const c_void) {

        let c_msg = msg_ptr.as_ref().expect("asSMessageInfo null");
        let _c_eng = params.as_ref().expect("engine params null");

        let script_engine: &mut ScriptEngine = &mut *(params as *mut ScriptEngine);
        
        if let Some(callback) = script_engine.msg_callback {
            let info = MessageInfo {
                section: read_cstring(c_msg.section),
                row: c_msg.row as u32,
                col: c_msg.col as u32,
                msg_type: c_msg.type_,
                message: read_cstring(c_msg.message)
            };

            callback(info);
        }
    }

    pub fn get_global_function_count(&self) -> u32 {
        
        let count = unsafe { asEngine_GetGlobalFunctionCount(self.engine) };
        return count as u32;
    }

    pub fn set_message_callback(&mut self, callback: ASMessageCallbackFunc) {

        self.msg_callback = Some(callback);

        type InternalCallback = Option<unsafe extern "C" fn(*const asSMessageInfo, *const c_void)>;        
        let base_func: InternalCallback = Some(crate::angelscript::engine::ScriptEngine::cvoid_msg_callback);
        let c_func = unsafe {std::mem::transmute::<InternalCallback, asFUNCTION_t>(base_func) };
        let c_self: *mut c_void = self as *mut _ as *mut c_void;
        let _result = unsafe { asEngine_SetMessageCallback(self.engine, c_func, c_self, asECallConvTypes_asCALL_CDECL) };

        //TODO: do something with the result.
    }

    pub fn send_message(&mut self, section: &str, row: u32, col: u32, msg_type: asEMsgType, message: &str) {

        let c_section = CString::new(section).expect("CString::new failed");
        let c_message = CString::new(message).expect("CString::new failed");
        let _result = unsafe { asEngine_WriteMessage(self.engine, c_section.as_ptr(), row as c_int, col as c_int, msg_type, c_message.as_ptr()) };

        //TODO: do something with the result.
    }

}