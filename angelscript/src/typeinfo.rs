use angelscript_sys::c_types::*;
use angelscript_sys::c_typeinfo::*;

use crate::types::EScriptBool;

//use crate::types::EReturnCodes;
use crate::function::ScriptFunction;
//use crate::read_cstring;

use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;

pub struct TypeInfo {
    info: *mut asITypeInfo,
}

impl TypeInfo {
    
    pub(crate) fn new(c_typeinfo: *mut asITypeInfo) -> Option<TypeInfo> {
        
        match c_typeinfo.is_null() {
            true => return None,
            false => return Some( TypeInfo{ info: c_typeinfo } )
        }
    }

    pub fn get_factory_by_decl(&mut self, decl: &str) -> Option<ScriptFunction> {

        let c_decl = CString::new(decl).unwrap();
        let c_function = unsafe { asTypeInfo_GetFactoryByDecl(self.info, c_decl.as_ptr()) };
        return ScriptFunction::new(c_function);
    }

    pub fn get_method_by_decl(&mut self, decl: &str) -> Option<ScriptFunction> {

        let c_decl = CString::new(decl).unwrap();
        let c_function = unsafe { asTypeInfo_GetMethodByDecl(self.info, c_decl.as_ptr(), EScriptBool::True.to_u32()) };
        return ScriptFunction::new(c_function);
    }

}