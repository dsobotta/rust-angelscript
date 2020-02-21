#![allow(dead_code)]

use crate::c_types::*;

extern "C" {

    pub fn asGeneric_GetEngine(
        g: *mut asIScriptGeneric
    ) -> *mut asIScriptEngine;

    pub fn asGeneric_GetFunction(
        g: *mut asIScriptGeneric
    ) -> *mut asIScriptFunction;

    pub fn asGeneric_GetAuxiliary(
        g: *mut asIScriptGeneric
    ) -> *mut ::std::os::raw::c_void;

    pub fn asGeneric_GetObject(
        g: *mut asIScriptGeneric
    ) -> *mut ::std::os::raw::c_void;

    pub fn asGeneric_GetObjectTypeId(
        g: *mut asIScriptGeneric
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_GetArgCount(
        g: *mut asIScriptGeneric
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_GetArgTypeId(
        g: *mut asIScriptGeneric,
        arg: asUINT,
        flags: *mut asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_GetArgByte(
        g: *mut asIScriptGeneric,
        arg: asUINT
    ) -> asBYTE;

    pub fn asGeneric_GetArgWord(
        g: *mut asIScriptGeneric,
        arg: asUINT
    ) -> asWORD;

    pub fn asGeneric_GetArgDWord(
        g: *mut asIScriptGeneric,
        arg: asUINT
    ) -> asDWORD;

    pub fn asGeneric_GetArgQWord(
        g: *mut asIScriptGeneric,
        arg: asUINT
    ) -> asQWORD;

    pub fn asGeneric_GetArgFloat(
        g: *mut asIScriptGeneric,
        arg: asUINT
    ) -> f32;

    pub fn asGeneric_GetArgDouble(
        g: *mut asIScriptGeneric,
        arg: asUINT
    ) -> f64;

    pub fn asGeneric_GetArgAddress(
        g: *mut asIScriptGeneric,
        arg: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asGeneric_GetArgObject(
        g: *mut asIScriptGeneric,
        arg: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asGeneric_GetAddressOfArg(
        g: *mut asIScriptGeneric,
        arg: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asGeneric_GetReturnTypeId(
        g: *mut asIScriptGeneric,
        flags: *mut asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnByte(
        g: *mut asIScriptGeneric,
        val: asBYTE
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnWord(
        g: *mut asIScriptGeneric,
        val: asWORD
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnDWord(
        g: *mut asIScriptGeneric,
        val: asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnQWord(
        g: *mut asIScriptGeneric,
        val: asQWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnFloat(
        g: *mut asIScriptGeneric,
        val: f32
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnDouble(
        g: *mut asIScriptGeneric,
        val: f64
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnAddress(
        g: *mut asIScriptGeneric,
        addr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_SetReturnObject(
        g: *mut asIScriptGeneric,
        obj: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asGeneric_GetAddressOfReturnLocation(
        g: *mut asIScriptGeneric,
    ) -> *mut ::std::os::raw::c_void;

}