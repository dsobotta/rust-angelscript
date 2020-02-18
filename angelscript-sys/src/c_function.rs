#![allow(dead_code)]

use crate::c_types::*;

 extern "C" {
 
    pub fn asFunction_GetEngine(
        f: *const asIScriptFunction
    ) -> *mut asIScriptEngine;

    pub fn asFunction_AddRef(
        f: *const asIScriptFunction
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_Release(
        f: *const asIScriptFunction
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_GetId(
        f: *const asIScriptFunction
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_GetFuncType(
        f: *const asIScriptFunction
    ) -> asEFuncType;

    pub fn asFunction_GetModuleName(
        f: *const asIScriptFunction
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_GetModule(
        f: *const asIScriptFunction
    ) -> *mut asIScriptModule;

    pub fn asFunction_GetScriptSectionName(
        f: *const asIScriptFunction,
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_GetConfigGroup(
        f: *const asIScriptFunction
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_GetAccessMask(
        f: *const asIScriptFunction
    ) -> asDWORD;

    pub fn asFunction_GetAuxiliary(
        f: *const asIScriptFunction
    ) -> *mut ::std::os::raw::c_void;

    pub fn asFunction_GetObjectType(
        f: *const asIScriptFunction
    ) -> *mut asITypeInfo;

    pub fn asFunction_GetObjectName(
        f: *const asIScriptFunction
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_GetName(
        f: *const asIScriptFunction
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_GetNamespace(
        f: *const asIScriptFunction
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_GetDeclaration(
        f: *const asIScriptFunction,
        includeObjectName: asBOOL,
        includeNamespace: asBOOL,
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_IsReadOnly(
        f: *const asIScriptFunction
    ) -> asBOOL;

    pub fn asFunction_IsPrivate(
        f: *const asIScriptFunction
    ) -> asBOOL;

    pub fn asFunction_IsProtected(
        f: *const asIScriptFunction
    ) -> asBOOL;

    pub fn asFunction_IsFinal(
        f: *const asIScriptFunction
    ) -> asBOOL;

    pub fn asFunction_IsOverride(
        f: *const asIScriptFunction
    ) -> asBOOL;

    pub fn asFunction_IsShared(
        f: *const asIScriptFunction
    ) -> asBOOL;

    pub fn asFunction_GetParamCount(
        f: *const asIScriptFunction
    ) -> asUINT;

    pub fn asFunction_GetParam(
        f: *const asIScriptFunction,
        index: asUINT,
        typeId: *mut ::std::os::raw::c_int,
        flags: *mut asDWORD,
        name: *mut *const ::std::os::raw::c_char,
        defaultArg: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_GetReturnTypeId(
        f: *const asIScriptFunction
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_GetTypeId(
        f: *const asIScriptFunction
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_IsCompatibleWithTypeId(
        f: *const asIScriptFunction,
        typeId: ::std::os::raw::c_int,
    ) -> asBOOL;

    pub fn asFunction_GetDelegateObject(
        f: *const asIScriptFunction
    ) -> *mut ::std::os::raw::c_void;

    pub fn asFunction_GetDelegateObjectType(
        f: *const asIScriptFunction
    ) -> *mut asITypeInfo;

    pub fn asFunction_GetDelegateFunction(
        f: *const asIScriptFunction
    ) -> *mut asIScriptFunction;

    pub fn asFunction_GetVarCount(
        f: *const asIScriptFunction
    ) -> asUINT;

    pub fn asFunction_GetVar(
        f: *const asIScriptFunction,
        index: asUINT,
        name: *mut *const ::std::os::raw::c_char,
        typeId: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_GetVarDecl(
        f: *const asIScriptFunction,
        index: asUINT,
        includeNamespace: asBOOL,
    ) -> *const ::std::os::raw::c_char;

    pub fn asFunction_FindNextLineWithCode(
        f: *const asIScriptFunction,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asFunction_GetByteCode(
        f: *mut asIScriptFunction,
        length: *mut asUINT
    ) -> *mut asDWORD;

    pub fn asFunction_SetUserData(
        f: *mut asIScriptFunction,
        userData: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asFunction_GetUserData(
        f: *const asIScriptFunction
    ) -> *mut ::std::os::raw::c_void;
      
 }