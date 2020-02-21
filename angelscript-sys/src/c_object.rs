#![allow(dead_code)]

use crate::c_types::*;

extern "C" {

    pub fn asObject_AddRef(
        s: *mut asIScriptObject
    ) -> ::std::os::raw::c_int;

    pub fn asObject_Release(
        s: *mut asIScriptObject
    ) -> ::std::os::raw::c_int;

    pub fn asObject_GetWeakRefFlag(
        s: *mut asIScriptObject
    ) -> *mut asILockableSharedBool;

    pub fn asObject_GetTypeId(
        s: *mut asIScriptObject
    ) -> ::std::os::raw::c_int;

    pub fn asObject_GetObjectType(
        s: *mut asIScriptObject
    ) -> *mut asITypeInfo;

    pub fn asObject_GetPropertyCount(
        s: *mut asIScriptObject
    ) -> asUINT;

    pub fn asObject_GetPropertyTypeId(
        s: *mut asIScriptObject,
        prop: asUINT,
    ) -> ::std::os::raw::c_int;

    pub fn asObject_GetPropertyName(
        s: *mut asIScriptObject,
        prop: asUINT,
    ) -> *const ::std::os::raw::c_char;

    pub fn asObject_GetAddressOfProperty(
        s: *mut asIScriptObject,
        prop: asUINT,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asObject_GetEngine(
        s: *mut asIScriptObject
    ) -> *mut asIScriptEngine;

    pub fn asObject_CopyFrom(
        s: *mut asIScriptObject,
        other: *mut asIScriptObject,
    ) -> ::std::os::raw::c_int;

    pub fn asObject_SetUserData(
        s: *mut asIScriptObject,
        data: *mut ::std::os::raw::c_void,
        type_: asPWORD,
    ) -> *mut ::std::os::raw::c_void;

    pub fn asObject_GetUserData(
        s: *mut asIScriptObject,
        type_: asPWORD,
    ) -> *mut ::std::os::raw::c_void;

}