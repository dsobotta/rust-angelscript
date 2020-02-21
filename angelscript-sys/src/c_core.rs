#![allow(dead_code)]

use crate::c_types::*;

extern "C" {

    pub fn asCreateScriptEngine(
        version: asDWORD
    ) -> *mut asIScriptEngine;

    pub fn asGetLibraryVersion(        
    ) -> *const ::std::os::raw::c_char;

    pub fn asGetLibraryOptions(        
    ) -> *const ::std::os::raw::c_char;

    pub fn asGetActiveContext(        
    ) -> *mut asIScriptContext;

    pub fn asPrepareMultithread(
        externalMgr: *mut asIThreadManager
    ) -> ::std::os::raw::c_int;

    pub fn asUnprepareMultithread(        
    );

    pub fn asGetThreadManager(        
    ) -> *mut asIThreadManager;

    pub fn asAcquireExclusiveLock(        
    );

    pub fn asReleaseExclusiveLock(        
    );

    pub fn asAcquireSharedLock(        
    );

    pub fn asReleaseSharedLock(        
    );

    pub fn asThreadCleanup(        
    ) -> ::std::os::raw::c_int;

    pub fn asSetGlobalMemoryFunctions(
        allocFunc: asALLOCFUNC_t,
        freeFunc: asFREEFUNC_t,
    ) -> ::std::os::raw::c_int;

    pub fn asResetGlobalMemoryFunctions(        
    ) -> ::std::os::raw::c_int;

    pub fn asAllocMem(
        size: usize
    ) -> *mut ::std::os::raw::c_void;

    pub fn asFreeMem(
        mem: *mut ::std::os::raw::c_void
    );

    pub fn asCreateLockableSharedBool(        
    ) -> *mut asILockableSharedBool;

}
