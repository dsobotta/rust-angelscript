#![allow(dead_code)]

use crate::c_types::*;

extern "C" {

    pub fn asModule_GetEngine(
        m: *mut asIScriptModule
    ) -> *mut asIScriptEngine;

    pub fn asModule_SetName(
        m: *mut asIScriptModule, 
        name: *const ::std::os::raw::c_char
    );

    pub fn asModule_GetName(
        m: *mut asIScriptModule
    ) -> *const ::std::os::raw::c_char;

    pub fn asModule_Discard(
        m: *mut asIScriptModule
    );

    pub fn asModule_AddScriptSection(
        m: *mut asIScriptModule,
        name: *const ::std::os::raw::c_char,
        code: *const ::std::os::raw::c_char,
        codeLength: usize,
        lineOffset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_Build(
        m: *mut asIScriptModule
    ) -> ::std::os::raw::c_int;

    pub fn asModule_CompileFunction(
        m: *mut asIScriptModule,
        sectionName: *const ::std::os::raw::c_char,
        code: *const ::std::os::raw::c_char,
        lineOffset: ::std::os::raw::c_int,
        compileFlags: asDWORD,
        outFunc: *mut *mut asIScriptFunction,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_CompileGlobalVar(
        m: *mut asIScriptModule,
        sectionName: *const ::std::os::raw::c_char,
        code: *const ::std::os::raw::c_char,
        lineOffset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_SetAccessMask(
        m: *mut asIScriptModule, 
        accessMask: asDWORD
    ) -> asDWORD;

    pub fn asModule_SetDefaultNamespace(
        m: *mut asIScriptModule,
        nameSpace: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetDefaultNamespace(
        m: *mut asIScriptModule
    ) -> *const ::std::os::raw::c_char;

    pub fn asModule_GetFunctionCount(
        m: *mut asIScriptModule
    ) -> asUINT;

    pub fn asModule_GetFunctionByIndex(
        m: *mut asIScriptModule,
        index: asUINT,
    ) -> *mut asIScriptFunction;

    pub fn asModule_GetFunctionByDecl(
        m: *mut asIScriptModule,
        decl: *const ::std::os::raw::c_char,
    ) -> *mut asIScriptFunction;

    pub fn asModule_GetFunctionByName(
        m: *mut asIScriptModule,
        name: *const ::std::os::raw::c_char,
    ) -> *mut asIScriptFunction;

    pub fn asModule_RemoveFunction(
        m: *mut asIScriptModule,
        func: *mut asIScriptFunction,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_ResetGlobalVars(
        m: *mut asIScriptModule,
        ctx: *mut asIScriptContext,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetGlobalVarCount(
        m: *mut asIScriptModule
    ) -> asUINT;

    pub fn asModule_GetGlobalVarIndexByName(
        m: *mut asIScriptModule,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetGlobalVarIndexByDecl(
        m: *mut asIScriptModule,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetGlobalVarDeclaration(
        m: *mut asIScriptModule,
        index: asUINT,
        includeNamespace: asBOOL,
    ) -> *const ::std::os::raw::c_char;

    pub fn asModule_GetGlobalVar(
        m: *mut asIScriptModule,
        index: asUINT,
        name: *mut *const ::std::os::raw::c_char,
        nameSpace: *mut *const ::std::os::raw::c_char,
        typeId: *mut ::std::os::raw::c_int,
        isConst: *mut asBOOL,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetAddressOfGlobalVar(
        m: *mut asIScriptModule,
        index: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asModule_RemoveGlobalVar(
        m: *mut asIScriptModule,
        index: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetObjectTypeCount(
        m: *mut asIScriptModule
    ) -> asUINT;

    pub fn asModule_GetObjectTypeByIndex(
        m: *mut asIScriptModule,
        index: asUINT,
    ) -> *mut asITypeInfo;

    pub fn asModule_GetTypeIdByDecl(
        m: *mut asIScriptModule,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetTypeInfoByName(
        m: *mut asIScriptModule,
        name: *const ::std::os::raw::c_char,
    ) -> *mut asITypeInfo;

    pub fn asModule_GetTypeInfoByDecl(
        m: *mut asIScriptModule,
        decl: *const ::std::os::raw::c_char,
    ) -> *mut asITypeInfo;


    pub fn asModule_GetEnumCount(
        m: *mut asIScriptModule
    ) -> asUINT;

    pub fn asModule_GetEnumByIndex(
        m: *mut asIScriptModule, index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asModule_GetTypedefCount(
        m: *mut asIScriptModule
    ) -> asUINT;

    pub fn asModule_GetTypedefByIndex(
        m: *mut asIScriptModule, index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asModule_GetImportedFunctionCount(
        m: *mut asIScriptModule
    ) -> asUINT;

    pub fn asModule_GetImportedFunctionIndexByDecl(
        m: *mut asIScriptModule,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_GetImportedFunctionDeclaration(
        m: *mut asIScriptModule,
        importIndex: asUINT,
    ) -> *const ::std::os::raw::c_char;

    pub fn asModule_GetImportedFunctionSourceModule(
        m: *mut asIScriptModule,
        importIndex: asUINT,
    ) -> *const ::std::os::raw::c_char;

    pub fn asModule_BindImportedFunction(
        m: *mut asIScriptModule,
        importIndex: asUINT,
        func: *mut asIScriptFunction,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_UnbindImportedFunction(
        m: *mut asIScriptModule,
        importIndex: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_BindAllImportedFunctions(
        m: *mut asIScriptModule
    ) -> ::std::os::raw::c_int;

    pub fn asModule_UnbindAllImportedFunctions(
        m: *mut asIScriptModule
    ) -> ::std::os::raw::c_int;

    pub fn asModule_SaveByteCode(
        m: *mut asIScriptModule,
        out: *mut asIBinaryStream,
        stripDebugInfo: asBOOL,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_LoadByteCode(
        m: *mut asIScriptModule,
        in_: *mut asIBinaryStream,
        wasDebugInfoStripped: *mut asBOOL,
    ) -> ::std::os::raw::c_int;

    pub fn asModule_SetUserData(
        m: *mut asIScriptModule,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asModule_GetUserData(
        m: *mut asIScriptModule
    ) -> *mut ::std::os::raw::c_void;

}
