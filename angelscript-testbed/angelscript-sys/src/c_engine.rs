#![allow(dead_code)]

use crate::c_types::*;

extern "C" {

    pub fn asCreateScriptEngine(
        version: asDWORD
    ) -> *mut asIScriptEngine;

    pub fn asEngine_AddRef(
        e: *mut asIScriptEngine
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_Release(
        e: *mut asIScriptEngine
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_ShutDownAndRelease(
        e: *mut asIScriptEngine
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_SetEngineProperty(
        e: *mut asIScriptEngine,
        property: asEEngineProp,
        value: asPWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetEngineProperty(
        e: *mut asIScriptEngine, 
        property: asEEngineProp
    ) -> asPWORD;

    pub fn asEngine_SetMessageCallback(
        e: *mut asIScriptEngine,
        callback: asFUNCTION_t,
        obj: *mut ::std::os::raw::c_void,
        callConv: asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_ClearMessageCallback(
        e: *mut asIScriptEngine
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_WriteMessage(
        e: *mut asIScriptEngine,
        section: *const ::std::os::raw::c_char,
        row: ::std::os::raw::c_int,
        col: ::std::os::raw::c_int,
        type_: asEMsgType,
        message: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_SetJITCompiler(
        e: *mut asIScriptEngine,
        compiler: *mut asIJITCompiler,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetJITCompiler(
        e: *mut asIScriptEngine
    ) -> *mut asIJITCompiler;

    pub fn asEngine_RegisterGlobalFunction(
        e: *mut asIScriptEngine,
        declaration: *const ::std::os::raw::c_char,
        funcPointer: asFUNCTION_t,
        callConv: asDWORD,
        auxiliary: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetGlobalFunctionCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetGlobalFunctionByIndex(
        e: *mut asIScriptEngine,
        index: asUINT,
    ) -> *mut asIScriptFunction;

    pub fn asEngine_GetGlobalFunctionByDecl(
        e: *mut asIScriptEngine,
        declaration: *const ::std::os::raw::c_char,
    ) -> *mut asIScriptFunction;

    pub fn asEngine_RegisterGlobalProperty(
        e: *mut asIScriptEngine,
        declaration: *const ::std::os::raw::c_char,
        pointer: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetGlobalPropertyCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetGlobalPropertyByIndex(
        e: *mut asIScriptEngine,
        index: asUINT,
        name: *mut *const ::std::os::raw::c_char,
        nameSpace: *mut *const ::std::os::raw::c_char,
        typeId: *mut ::std::os::raw::c_int,
        isConst: *mut asBOOL,
        configGroup: *mut *const ::std::os::raw::c_char,
        pointer: *mut *mut ::std::os::raw::c_void,
        accessMask: *mut asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetGlobalPropertyIndexByName(
        e: *mut asIScriptEngine,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetGlobalPropertyIndexByDecl(
        e: *mut asIScriptEngine,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterObjectType(
        e: *mut asIScriptEngine,
        name: *const ::std::os::raw::c_char,
        byteSize: ::std::os::raw::c_int,
        flags: asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterObjectProperty(
        e: *mut asIScriptEngine,
        obj: *const ::std::os::raw::c_char,
        declaration: *const ::std::os::raw::c_char,
        byteOffset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterObjectMethod(
        e: *mut asIScriptEngine,
        obj: *const ::std::os::raw::c_char,
        declaration: *const ::std::os::raw::c_char,
        funcPointer: asFUNCTION_t,
        callConv: asDWORD,
        auxiliary: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterObjectBehaviour(
        e: *mut asIScriptEngine,
        datatype: *const ::std::os::raw::c_char,
        behaviour: asEBehaviours,
        declaration: *const ::std::os::raw::c_char,
        funcPointer: asFUNCTION_t,
        callConv: asDWORD,
        auxiliary: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterInterface(
        e: *mut asIScriptEngine,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterInterfaceMethod(
        e: *mut asIScriptEngine,
        intf: *const ::std::os::raw::c_char,
        declaration: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetObjectTypeCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetObjectTypeByIndex(
        e: *mut asIScriptEngine,
        index: asUINT,
    ) -> *mut asITypeInfo;

    pub fn asEngine_RegisterStringFactory(
        e: *mut asIScriptEngine,
        datatype: *const ::std::os::raw::c_char,
        factoryFunc: asFUNCTION_t,
        callConv: asDWORD,
        auxiliary: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetStringFactoryReturnTypeId(
        e: *mut asIScriptEngine,
        flags: *mut asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterDefaultArrayType(
        e: *mut asIScriptEngine,
        type_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetDefaultArrayTypeId(
        e: *mut asIScriptEngine
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterEnum(
        e: *mut asIScriptEngine,
        type_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RegisterEnumValue(
        e: *mut asIScriptEngine,
        type_: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetEnumCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetEnumByIndex(
        e: *mut asIScriptEngine, 
        index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asEngine_RegisterFuncdef(
        e: *mut asIScriptEngine,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetFuncdefCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetFuncdefByIndex(
        e: *mut asIScriptEngine, 
        index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asEngine_RegisterTypedef(
        e: *mut asIScriptEngine,
        type_: *const ::std::os::raw::c_char,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetTypedefCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetTypedefByIndex(
        e: *mut asIScriptEngine, 
        index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asEngine_BeginConfigGroup(
        e: *mut asIScriptEngine,
        groupName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_EndConfigGroup(
        e: *mut asIScriptEngine
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_RemoveConfigGroup(
        e: *mut asIScriptEngine,
        groupName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_SetDefaultAccessMask(
        e: *mut asIScriptEngine, 
        defaultMask: asDWORD
    ) -> asDWORD;

    pub fn asEngine_SetDefaultNamespace(
        e: *mut asIScriptEngine,
        nameSpace: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetDefaultNamespace(
        e: *mut asIScriptEngine
    ) -> *const ::std::os::raw::c_char;

    pub fn asEngine_GetModule(
        e: *mut asIScriptEngine,
        module: *const ::std::os::raw::c_char,
        flag: asEGMFlags,
    ) -> *mut asIScriptModule;

    pub fn asEngine_DiscardModule(
        e: *mut asIScriptEngine,
        module: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetModuleCount(
        e: *mut asIScriptEngine
    ) -> asUINT;

    pub fn asEngine_GetModuleByIndex(
        e: *mut asIScriptEngine,
        index: asUINT,
    ) -> *mut asIScriptModule;

    pub fn asEngine_GetFunctionById(
        e: *mut asIScriptEngine,
        funcId: ::std::os::raw::c_int,
    ) -> *mut asIScriptFunction;

    pub fn asEngine_GetTypeIdByDecl(
        e: *mut asIScriptEngine,
        decl: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetTypeDeclaration(
        e: *mut asIScriptEngine,
        typeId: ::std::os::raw::c_int,
        includeNamespace: asBOOL,
    ) -> *const ::std::os::raw::c_char;

    pub fn asEngine_GetSizeOfPrimitiveType(
        e: *mut asIScriptEngine,
        typeId: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetTypeInfoById(
        e: *mut asIScriptEngine,
        typeId: ::std::os::raw::c_int,
    ) -> *mut asITypeInfo;

    pub fn asEngine_GetTypeInfoByName(
        e: *mut asIScriptEngine,
        name: *const ::std::os::raw::c_char,
    ) -> *mut asITypeInfo;

    pub fn asEngine_GetTypeInfoByDecl(
        e: *mut asIScriptEngine,
        decl: *const ::std::os::raw::c_char,
    ) -> *mut asITypeInfo;

    pub fn asEngine_CreateContext(
        e: *mut asIScriptEngine
    ) -> *mut asIScriptContext;

    pub fn asEngine_CreateScriptObject(
        e: *mut asIScriptEngine,
        type_: *mut asITypeInfo,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asEngine_CreateScriptObjectCopy(
        e: *mut asIScriptEngine,
        obj: *mut ::std::os::raw::c_void,
        type_: *mut asITypeInfo,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asEngine_CreateUninitializedScriptObject(
        e: *mut asIScriptEngine,
        type_: *mut asITypeInfo,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asEngine_CreateDelegate(
        e: *mut asIScriptEngine,
        func: *mut asIScriptFunction,
        obj: *mut ::std::os::raw::c_void,
    ) -> *mut asIScriptFunction;

    pub fn asEngine_AssignScriptObject(
        e: *mut asIScriptEngine,
        dstObj: *mut ::std::os::raw::c_void,
        srcObj: *mut ::std::os::raw::c_void,
        type_: *mut asITypeInfo,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_ReleaseScriptObject(
        e: *mut asIScriptEngine,
        obj: *mut ::std::os::raw::c_void,
        type_: *mut asITypeInfo,
    );

    pub fn asEngine_AddRefScriptObject(
        e: *mut asIScriptEngine,
        obj: *mut ::std::os::raw::c_void,
        type_: *mut asITypeInfo,
    );

    pub fn asEngine_GetWeakRefFlagOfScriptObject(
        e: *mut asIScriptEngine,
        obj: *mut ::std::os::raw::c_void,
        type_: *mut asITypeInfo,
    ) -> *mut asILockableSharedBool;

    pub fn asEngine_RequestContext(
        e: *mut asIScriptEngine
    ) -> *mut asIScriptContext;

    pub fn asEngine_ReturnContext(
        e: *mut asIScriptEngine, 
        ctx: *mut asIScriptContext
    );

    pub fn asEngine_SetContextCallbacks(
        e: *mut asIScriptEngine,
        requestCtx: asREQUESTCONTEXTFUNC_t,
        returnCtx: asRETURNCONTEXTFUNC_t,
        param: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_ParseToken(
        e: *mut asIScriptEngine,
        string: *const ::std::os::raw::c_char,
        stringLength: usize,
        tokenLength: *mut asUINT,
    ) -> asETokenClass;

    pub fn asEngine_GarbageCollect(
        e: *mut asIScriptEngine,
        flags: asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetGCStatistics(
        e: *mut asIScriptEngine,
        currentSize: *mut asUINT,
        totalDestroyed: *mut asUINT,
        totalDetected: *mut asUINT,
        newObjects: *mut asUINT,
        totalNewDestroyed: *mut asUINT,
    );

    pub fn asEngine_NotifyGarbageCollectorOfNewObject(
        e: *mut asIScriptEngine,
        obj: *mut ::std::os::raw::c_void,
        type_: *mut asITypeInfo,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GetObjectInGC(
        e: *mut asIScriptEngine,
        idx: asUINT,
        seqNbr: *mut asUINT,
        obj: *mut *mut ::std::os::raw::c_void,
        type_: *mut *mut asITypeInfo,
    ) -> ::std::os::raw::c_int;

    pub fn asEngine_GCEnumCallback(
        e: *mut asIScriptEngine, 
        obj: *mut ::std::os::raw::c_void
    );

    pub fn asEngine_SetUserData(
        e: *mut asIScriptEngine,
        data: *mut ::std::os::raw::c_void,
        type_: asPWORD,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asEngine_GetUserData(
        e: *mut asIScriptEngine,
        type_: asPWORD,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asEngine_SetEngineUserDataCleanupCallback(
        e: *mut asIScriptEngine,
        callback: asCLEANENGINEFUNC_t,
        type_: asPWORD,
    );

    pub fn asEngine_SetModuleUserDataCleanupCallback(
        e: *mut asIScriptEngine,
        callback: asCLEANMODULEFUNC_t,
    );

    pub fn asEngine_SetContextUserDataCleanupCallback(
        e: *mut asIScriptEngine,
        callback: asCLEANCONTEXTFUNC_t,
    );

    pub fn asEngine_SetFunctionUserDataCleanupCallback(
        e: *mut asIScriptEngine,
        callback: asCLEANFUNCTIONFUNC_t,
    );

    pub fn asEngine_SetTypeInfoUserDataCleanupCallback(
        e: *mut asIScriptEngine,
        callback: asCLEANTYPEINFOFUNC_t,
        type_: asPWORD,
    );

    pub fn asEngine_SetScriptObjectUserDataCleanupCallback(
        e: *mut asIScriptEngine,
        callback: asCLEANSCRIPTOBJECTFUNC_t,
        type_: asPWORD,
    );

}