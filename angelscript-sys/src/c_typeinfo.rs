#![allow(dead_code)]

use crate::c_types::*;

extern "C" {

    pub fn asTypeInfo_GetEngine(
        o: *mut asITypeInfo
    ) -> *mut asIScriptEngine;

    pub fn asTypeInfo_GetConfigGroup(
        o: *mut asITypeInfo
    ) -> *const ::std::os::raw::c_char;

    pub fn asTypeInfo_GetAccessMask(
        o: *mut asITypeInfo
    ) -> asDWORD;

    pub fn asTypeInfo_GetModule(
        o: *mut asITypeInfo
    ) -> *mut asIScriptModule;

    pub fn asTypeInfo_AddRef(
        o: *const asITypeInfo
    ) -> ::std::os::raw::c_int;

    pub fn asTypeInfo_Release(
        o: *const asITypeInfo
    ) -> ::std::os::raw::c_int;

    pub fn asTypeInfo_GetName(
        o: *const asITypeInfo
    ) -> *const ::std::os::raw::c_char;

    pub fn asTypeInfo_GetNamespace(
        o: *const asITypeInfo
    ) -> *const ::std::os::raw::c_char;

    pub fn asTypeInfo_GetBaseType(
        o: *const asITypeInfo
    ) -> *mut asITypeInfo;

    pub fn asTypeInfo_DerivesFrom(
        o: *const asITypeInfo,
        objType: *const asITypeInfo
    ) -> asBOOL;

    pub fn asTypeInfo_GetFlags(
        o: *const asITypeInfo
    ) -> asDWORD;

    pub fn asTypeInfo_GetSize(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetTypeId(
        o: *const asITypeInfo
    ) -> ::std::os::raw::c_int;

    pub fn asTypeInfo_GetSubTypeId(
        o: *const asITypeInfo,
        subTypeIndex: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asTypeInfo_GetSubType(
        o: *const asITypeInfo,
        subTypeIndex: asUINT
    ) -> *mut asITypeInfo;

    pub fn asTypeInfo_GetSubTypeCount(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetInterfaceCount(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetInterface(
        o: *const asITypeInfo,
        index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asTypeInfo_Implements(
        o: *const asITypeInfo,
        objType: *const asITypeInfo
    ) -> asBOOL;

    pub fn asTypeInfo_GetFactoryCount(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetFactoryByIndex(
        o: *const asITypeInfo,
        index: asUINT,
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_GetFactoryByDecl(
        o: *const asITypeInfo,
        decl: *const ::std::os::raw::c_char,
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_GetMethodCount(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetMethodByIndex(
        o: *const asITypeInfo,
        index: asUINT,
        getVirtual: asBOOL,
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_GetMethodByName(
        o: *const asITypeInfo,
        name: *const ::std::os::raw::c_char,
        getVirtual: asBOOL,
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_GetMethodByDecl(
        o: *const asITypeInfo,
        decl: *const ::std::os::raw::c_char,
        getVirtual: asBOOL,
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_GetPropertyCount(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetProperty(
        o: *const asITypeInfo,
        index: asUINT,
        name: *mut *const ::std::os::raw::c_char,
        typeId: *mut ::std::os::raw::c_int,
        isPrivate: *mut asBOOL,
        isProtected: *mut asBOOL,
        offset: *mut ::std::os::raw::c_int,
        isReference: *mut asBOOL,
        accessMask: *mut asDWORD,
    ) -> ::std::os::raw::c_int;

    pub fn asTypeInfo_GetPropertyDeclaration(
        o: *const asITypeInfo,
        index: asUINT,
    ) -> *const ::std::os::raw::c_char;

    pub fn asTypeInfo_GetBehaviourCount(
        o: *const asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetBehaviourByIndex(
        o: *const asITypeInfo,
        index: asUINT,
        outBehaviour: *mut asEBehaviours,
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_GetChildFuncdefCount(
        o: *mut asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetChildFuncdef(
        o: *mut asITypeInfo,
        index: asUINT
    ) -> *mut asITypeInfo;

    pub fn asTypeInfo_GetParentType(
        o: *mut asITypeInfo
    ) -> *mut asITypeInfo;

    pub fn asTypeInfo_GetEnumValueCount(
        o: *mut asITypeInfo
    ) -> asUINT;

    pub fn asTypeInfo_GetEnumValueByIndex(
        o: *mut asITypeInfo,
        index: asUINT,
        outValue: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;

    pub fn asTypeInfo_GetTypedefTypeId(
        o: *mut asITypeInfo
    ) -> ::std::os::raw::c_int;

    pub fn asTypeInfo_GetFuncdefSignature(
        o: *mut asITypeInfo
    ) -> *mut asIScriptFunction;

    pub fn asTypeInfo_SetUserData(
        o: *mut asITypeInfo,
        data: *mut ::std::os::raw::c_void,
        type_: asPWORD,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asTypeInfo_GetUserData(
        o: *mut asITypeInfo,
        type_: asPWORD,
    ) -> *mut ::std::os::raw::c_void;

}