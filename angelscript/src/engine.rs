use angelscript_sys::c_types::*;
use angelscript_sys::c_engine::*;

use crate::types::*;
use crate::read_cstring;
use crate::module::ScriptModule;
use crate::context::ScriptContext;

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

    pub fn set_message_callback(&mut self, callback: ASMessageCallbackFunc) -> EReturnCodes {

        self.msg_callback = Some(callback);

        type InternalCallback = Option<unsafe extern "C" fn(*const asSMessageInfo, *const c_void)>;        
        let base_func: InternalCallback = Some(crate::engine::ScriptEngine::cvoid_msg_callback);
        let c_func = unsafe {std::mem::transmute::<InternalCallback, asFUNCTION_t>(base_func) };
        let c_self: *mut c_void = self as *mut _ as *mut c_void;
        let result = unsafe { asEngine_SetMessageCallback(self.engine, c_func, c_self, ECallConvTypes::CDecl.to_u32()) };

        return EReturnCodes::from_i32(result);
    }

    pub fn send_message(&mut self, section: &str, row: u32, col: u32, msg_type: EMsgType, message: &str) -> EReturnCodes {

        let c_section = CString::new(section).unwrap();
        let c_message = CString::new(message).unwrap();
        let result = unsafe { asEngine_WriteMessage(self.engine, c_section.as_ptr(), row as c_int, col as c_int, msg_type.to_u32(), c_message.as_ptr()) };

        return EReturnCodes::from_i32(result);
    }

    pub fn get_module(&mut self, module: &str, flag: EGMFlags) -> Option<ScriptModule> {
        
        let c_module_name = CString::new(module).unwrap();
        let c_script_module = unsafe { asEngine_GetModule(self.engine, c_module_name.as_ptr(), flag.to_u32()) };
        return ScriptModule::new(c_script_module);
    }

    pub fn create_context(&mut self) -> Option<ScriptContext> {
        
        let c_context = unsafe { asEngine_CreateContext(self.engine) };
        return ScriptContext::new(c_context);
    }

    pub fn get_global_function_count(&self) -> u32 {
        
        let count = unsafe { asEngine_GetGlobalFunctionCount(self.engine) };
        return count as u32;
    }

}