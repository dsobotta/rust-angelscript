#![allow(dead_code)]

use crate::c_types::*;

extern "C" {
    
    pub fn asContext_AddRef(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_Release(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetEngine(
        c: *mut asIScriptContext
    ) -> *mut asIScriptEngine;

    pub fn asContext_Prepare(
        c: *mut asIScriptContext,
        func: *mut asIScriptFunction,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_Unprepare(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_Execute(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_Abort(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_Suspend(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetState(
        c: *mut asIScriptContext
    ) -> asEContextState;

    pub fn asContext_PushState(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_PopState(
        c: *mut asIScriptContext
    ) -> ::std::os::raw::c_int;

    pub fn asContext_IsNested(
        c: *mut asIScriptContext,
        nestCount: *mut asUINT
    ) -> asBOOL;

    pub fn asContext_SetObject(
        c: *mut asIScriptContext,
        obj: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgByte(
        c: *mut asIScriptContext,
        arg: asUINT,
        value: asBYTE,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgWord(
        c: *mut asIScriptContext,
        arg: asUINT,
        value: asWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgDWord(
        c: *mut asIScriptContext,
        arg: asUINT,
        value: asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgQWord(
        c: *mut asIScriptContext,
        arg: asUINT,
        value: asQWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgFloat(
        c: *mut asIScriptContext,
        arg: asUINT,
        value: f32,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgDouble(
        c: *mut asIScriptContext,
        arg: asUINT,
        value: f64,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgAddress(
        c: *mut asIScriptContext,
        arg: asUINT,
        addr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgObject(
        c: *mut asIScriptContext,
        arg: asUINT,
        obj: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_SetArgVarType(
        c: *mut asIScriptContext,
        arg: asUINT,
        ptr: *mut ::std::os::raw::c_void,
        typeId: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetAddressOfArg(
        c: *mut asIScriptContext,
        arg: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_GetReturnByte(
        c: *mut asIScriptContext
    ) -> asBYTE;

    pub fn asContext_GetReturnWord(
        c: *mut asIScriptContext
    ) -> asWORD;

    pub fn asContext_GetReturnDWord(
        c: *mut asIScriptContext
    ) -> asDWORD;

    pub fn asContext_GetReturnQWord(
        c: *mut asIScriptContext
    ) -> asQWORD;

    pub fn asContext_GetReturnFloat(
        c: *mut asIScriptContext
    ) -> f32;

    pub fn asContext_GetReturnDouble(
        c: *mut asIScriptContext
    ) -> f64;

    pub fn asContext_GetReturnAddress(
        c: *mut asIScriptContext
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_GetReturnObject(
        c: *mut asIScriptContext
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_GetAddressOfReturnValue(
        c: *mut asIScriptContext,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_SetException(
        c: *mut asIScriptContext,
        string: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetExceptionLineNumber(
        c: *mut asIScriptContext,
        column: *mut ::std::os::raw::c_int,
        sectionName: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetExceptionFunction(
        c: *mut asIScriptContext
    ) -> *mut asIScriptFunction;

    pub fn asContext_GetExceptionString(
        c: *mut asIScriptContext
    ) -> *const ::std::os::raw::c_char;

    pub fn asContext_SetExceptionCallback(
        c: *mut asIScriptContext,
        callback: asFUNCTION_t,
        obj: *mut ::std::os::raw::c_void,
        callConv: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_ClearExceptionCallback(
        c: *mut asIScriptContext
    );

    pub fn asContext_SetLineCallback(
        c: *mut asIScriptContext,
        callback: asFUNCTION_t,
        obj: *mut ::std::os::raw::c_void,
        callConv: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_ClearLineCallback(
        c: *mut asIScriptContext
    );

    pub fn asContext_GetCallstackSize(
        c: *mut asIScriptContext
    ) -> asUINT;

    pub fn asContext_GetFunction(
        c: *mut asIScriptContext,
        stackLevel: asUINT,
    ) -> *mut asIScriptFunction;

    pub fn asContext_GetLineNumber(
        c: *mut asIScriptContext,
        stackLevel: asUINT,
        column: *mut ::std::os::raw::c_int,
        sectionName: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetVarCount(
        c: *mut asIScriptContext,
        stackLevel: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetVarName(
        c: *mut asIScriptContext,
        varIndex: asUINT,
        stackLevel: asUINT,
    ) -> *const ::std::os::raw::c_char;

    pub fn asContext_GetVarDeclaration(
        c: *mut asIScriptContext,
        varIndex: asUINT,
        stackLevel: asUINT,
        includeNamespace: asBOOL,
    ) -> *const ::std::os::raw::c_char;

    pub fn asContext_GetVarTypeId(
        c: *mut asIScriptContext,
        varIndex: asUINT,
        stackLevel: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetAddressOfVar(
        c: *mut asIScriptContext,
        varIndex: asUINT,
        stackLevel: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_IsVarInScope(
        c: *mut asIScriptContext,
        varIndex: asUINT,
        stackLevel: asUINT,
    ) -> asBOOL;

    pub fn asContext_GetThisTypeId(
        c: *mut asIScriptContext,
        stackLevel: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asContext_GetThisPointer(
        c: *mut asIScriptContext,
        stackLevel: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_GetSystemFunction(
        c: *mut asIScriptContext
    ) -> *mut asIScriptFunction;

    pub fn asContext_SetUserData(
        c: *mut asIScriptContext,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asContext_GetUserData(
        c: *mut asIScriptContext
    ) -> *mut ::std::os::raw::c_void;

}