use angelscript_sys::c_types::*;
use angelscript_sys::c_module::*;

use crate::types::EReturnCodes;
use crate::function::ScriptFunction;
use crate::typeinfo::TypeInfo;
//use crate::read_cstring;

use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct ScriptModule {
    module: *mut asIScriptModule,
}

impl ScriptModule {
    
    pub(crate) fn new(c_module: *mut asIScriptModule) -> Option<ScriptModule> {
        
        match c_module.is_null() {
            true => return None,
            false => return Some( ScriptModule{ module: c_module } )
        }
    }

    pub fn add_script_section(&mut self, name: &str, code: &str) -> EReturnCodes {
        
        let c_name = CString::new(name).unwrap();
        let c_code = CString::new(code).unwrap();
        let result = unsafe { asModule_AddScriptSection(self.module, c_name.as_ptr(), c_code.as_ptr(), 0, 0) };

        return EReturnCodes::from_i32(result);
    }

    pub fn build(&mut self) -> EReturnCodes {
        
        let result = unsafe { asModule_Build(self.module) };
        return EReturnCodes::from_i32(result);
    }

    pub fn get_function_by_decl(&mut self, decl: &str) -> Option<ScriptFunction> {

        let c_decl = CString::new(decl).unwrap();
        let c_function = unsafe { asModule_GetFunctionByDecl(self.module, c_decl.as_ptr()) };
        return ScriptFunction::new(c_function);
    }

    pub fn get_typeinfo_by_decl(&mut self, decl: &str) -> Option<TypeInfo> {

        let c_decl = CString::new(decl).unwrap();
        let c_typeinfo = unsafe { asModule_GetTypeInfoByDecl(self.module, c_decl.as_ptr()) };
        return TypeInfo::new(c_typeinfo);
    }
}