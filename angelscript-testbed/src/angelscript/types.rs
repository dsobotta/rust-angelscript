#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const ANGELSCRIPT_VERSION: u32 = 23102;

pub type wchar_t = ::std::os::raw::c_int;
pub type asBYTE = ::std::os::raw::c_uchar;
pub type asWORD = ::std::os::raw::c_ushort;
pub type asUINT = ::std::os::raw::c_uint;
pub type asPWORD = usize;
pub type asDWORD = ::std::os::raw::c_uint;
pub type asQWORD = ::std::os::raw::c_ulong;
pub type asINT64 = ::std::os::raw::c_long;

pub const asBOOL_asTRUE: asBOOL = 1;
pub const asBOOL_asFALSE: asBOOL = 0;
pub type asBOOL = u32;

pub const asERetCodes_asSUCCESS: asERetCodes = 0;
pub const asERetCodes_asERROR: asERetCodes = -1;
pub const asERetCodes_asCONTEXT_ACTIVE: asERetCodes = -2;
pub const asERetCodes_asCONTEXT_NOT_FINISHED: asERetCodes = -3;
pub const asERetCodes_asCONTEXT_NOT_PREPARED: asERetCodes = -4;
pub const asERetCodes_asINVALID_ARG: asERetCodes = -5;
pub const asERetCodes_asNO_FUNCTION: asERetCodes = -6;
pub const asERetCodes_asNOT_SUPPORTED: asERetCodes = -7;
pub const asERetCodes_asINVALID_NAME: asERetCodes = -8;
pub const asERetCodes_asNAME_TAKEN: asERetCodes = -9;
pub const asERetCodes_asINVALID_DECLARATION: asERetCodes = -10;
pub const asERetCodes_asINVALID_OBJECT: asERetCodes = -11;
pub const asERetCodes_asINVALID_TYPE: asERetCodes = -12;
pub const asERetCodes_asALREADY_REGISTERED: asERetCodes = -13;
pub const asERetCodes_asMULTIPLE_FUNCTIONS: asERetCodes = -14;
pub const asERetCodes_asNO_MODULE: asERetCodes = -15;
pub const asERetCodes_asNO_GLOBAL_VAR: asERetCodes = -16;
pub const asERetCodes_asINVALID_CONFIGURATION: asERetCodes = -17;
pub const asERetCodes_asINVALID_INTERFACE: asERetCodes = -18;
pub const asERetCodes_asCANT_BIND_ALL_FUNCTIONS: asERetCodes = -19;
pub const asERetCodes_asLOWER_ARRAY_DIMENSION_NOT_REGISTERED: asERetCodes = -20;
pub const asERetCodes_asWRONG_CONFIG_GROUP: asERetCodes = -21;
pub const asERetCodes_asCONFIG_GROUP_IS_IN_USE: asERetCodes = -22;
pub const asERetCodes_asILLEGAL_BEHAVIOUR_FOR_TYPE: asERetCodes = -23;
pub const asERetCodes_asWRONG_CALLING_CONV: asERetCodes = -24;
pub const asERetCodes_asBUILD_IN_PROGRESS: asERetCodes = -25;
pub const asERetCodes_asINIT_GLOBAL_VARS_FAILED: asERetCodes = -26;
pub const asERetCodes_asOUT_OF_MEMORY: asERetCodes = -27;
pub const asERetCodes_asMODULE_IS_IN_USE: asERetCodes = -28;
pub type asERetCodes = i32;

pub const asEEngineProp_asEP_ALLOW_UNSAFE_REFERENCES: asEEngineProp = 1;
pub const asEEngineProp_asEP_OPTIMIZE_BYTECODE: asEEngineProp = 2;
pub const asEEngineProp_asEP_COPY_SCRIPT_SECTIONS: asEEngineProp = 3;
pub const asEEngineProp_asEP_MAX_STACK_SIZE: asEEngineProp = 4;
pub const asEEngineProp_asEP_USE_CHARACTER_LITERALS: asEEngineProp = 5;
pub const asEEngineProp_asEP_ALLOW_MULTILINE_STRINGS: asEEngineProp = 6;
pub const asEEngineProp_asEP_ALLOW_IMPLICIT_HANDLE_TYPES: asEEngineProp = 7;
pub const asEEngineProp_asEP_BUILD_WITHOUT_LINE_CUES: asEEngineProp = 8;
pub const asEEngineProp_asEP_INIT_GLOBAL_VARS_AFTER_BUILD: asEEngineProp = 9;
pub const asEEngineProp_asEP_REQUIRE_ENUM_SCOPE: asEEngineProp = 10;
pub const asEEngineProp_asEP_SCRIPT_SCANNER: asEEngineProp = 11;
pub const asEEngineProp_asEP_INCLUDE_JIT_INSTRUCTIONS: asEEngineProp = 12;
pub const asEEngineProp_asEP_STRING_ENCODING: asEEngineProp = 13;
pub const asEEngineProp_asEP_PROPERTY_ACCESSOR_MODE: asEEngineProp = 14;
pub const asEEngineProp_asEP_EXPAND_DEF_ARRAY_TO_TMPL: asEEngineProp = 15;
pub const asEEngineProp_asEP_AUTO_GARBAGE_COLLECT: asEEngineProp = 16;
pub const asEEngineProp_asEP_DISALLOW_GLOBAL_VARS: asEEngineProp = 17;
pub const asEEngineProp_asEP_ALWAYS_IMPL_DEFAULT_CONSTRUCT: asEEngineProp = 18;
pub const asEEngineProp_asEP_COMPILER_WARNINGS: asEEngineProp = 19;
pub const asEEngineProp_asEP_DISALLOW_VALUE_ASSIGN_FOR_REF_TYPE: asEEngineProp = 20;
pub const asEEngineProp_asEP_ALTER_SYNTAX_NAMED_ARGS: asEEngineProp = 21;
pub const asEEngineProp_asEP_DISABLE_INTEGER_DIVISION: asEEngineProp = 22;
pub const asEEngineProp_asEP_DISALLOW_EMPTY_LIST_ELEMENTS: asEEngineProp = 23;
pub const asEEngineProp_asEP_PRIVATE_PROP_AS_PROTECTED: asEEngineProp = 24;
pub const asEEngineProp_asEP_ALLOW_UNICODE_IDENTIFIERS: asEEngineProp = 25;
pub const asEEngineProp_asEP_HEREDOC_TRIM_MODE: asEEngineProp = 26;
pub const asEEngineProp_asEP_LAST_PROPERTY: asEEngineProp = 27;
pub type asEEngineProp = u32;

pub const asECallConvTypes_asCALL_CDECL: asECallConvTypes = 0;
pub const asECallConvTypes_asCALL_STDCALL: asECallConvTypes = 1;
pub const asECallConvTypes_asCALL_THISCALL_ASGLOBAL: asECallConvTypes = 2;
pub const asECallConvTypes_asCALL_THISCALL: asECallConvTypes = 3;
pub const asECallConvTypes_asCALL_CDECL_OBJLAST: asECallConvTypes = 4;
pub const asECallConvTypes_asCALL_CDECL_OBJFIRST: asECallConvTypes = 5;
pub const asECallConvTypes_asCALL_GENERIC: asECallConvTypes = 6;
pub const asECallConvTypes_asCALL_THISCALL_OBJLAST: asECallConvTypes = 7;
pub const asECallConvTypes_asCALL_THISCALL_OBJFIRST: asECallConvTypes = 8;
pub type asECallConvTypes = u32;

pub const asEObjTypeFlags_asOBJ_REF: asEObjTypeFlags = 1;
pub const asEObjTypeFlags_asOBJ_VALUE: asEObjTypeFlags = 2;
pub const asEObjTypeFlags_asOBJ_GC: asEObjTypeFlags = 4;
pub const asEObjTypeFlags_asOBJ_POD: asEObjTypeFlags = 8;
pub const asEObjTypeFlags_asOBJ_NOHANDLE: asEObjTypeFlags = 16;
pub const asEObjTypeFlags_asOBJ_SCOPED: asEObjTypeFlags = 32;
pub const asEObjTypeFlags_asOBJ_TEMPLATE: asEObjTypeFlags = 64;
pub const asEObjTypeFlags_asOBJ_ASHANDLE: asEObjTypeFlags = 128;
pub const asEObjTypeFlags_asOBJ_APP_CLASS: asEObjTypeFlags = 256;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CONSTRUCTOR: asEObjTypeFlags = 512;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_DESTRUCTOR: asEObjTypeFlags = 1024;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_ASSIGNMENT: asEObjTypeFlags = 2048;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_COPY_CONSTRUCTOR: asEObjTypeFlags = 4096;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_C: asEObjTypeFlags = 768;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CD: asEObjTypeFlags = 1792;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CA: asEObjTypeFlags = 2816;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CK: asEObjTypeFlags = 4864;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CDA: asEObjTypeFlags = 3840;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CDK: asEObjTypeFlags = 5888;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CAK: asEObjTypeFlags = 6912;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_CDAK: asEObjTypeFlags = 7936;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_D: asEObjTypeFlags = 1280;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_DA: asEObjTypeFlags = 3328;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_DK: asEObjTypeFlags = 5376;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_DAK: asEObjTypeFlags = 7424;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_A: asEObjTypeFlags = 2304;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_AK: asEObjTypeFlags = 6400;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_K: asEObjTypeFlags = 4352;
pub const asEObjTypeFlags_asOBJ_APP_PRIMITIVE: asEObjTypeFlags = 8192;
pub const asEObjTypeFlags_asOBJ_APP_FLOAT: asEObjTypeFlags = 16384;
pub const asEObjTypeFlags_asOBJ_APP_ARRAY: asEObjTypeFlags = 32768;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_ALLINTS: asEObjTypeFlags = 65536;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_ALLFLOATS: asEObjTypeFlags = 131072;
pub const asEObjTypeFlags_asOBJ_NOCOUNT: asEObjTypeFlags = 262144;
pub const asEObjTypeFlags_asOBJ_APP_CLASS_ALIGN8: asEObjTypeFlags = 524288;
pub const asEObjTypeFlags_asOBJ_IMPLICIT_HANDLE: asEObjTypeFlags = 1048576;
pub const asEObjTypeFlags_asOBJ_MASK_VALID_FLAGS: asEObjTypeFlags = 2097151;
pub const asEObjTypeFlags_asOBJ_SCRIPT_OBJECT: asEObjTypeFlags = 2097152;
pub const asEObjTypeFlags_asOBJ_SHARED: asEObjTypeFlags = 4194304;
pub const asEObjTypeFlags_asOBJ_NOINHERIT: asEObjTypeFlags = 8388608;
pub const asEObjTypeFlags_asOBJ_FUNCDEF: asEObjTypeFlags = 16777216;
pub const asEObjTypeFlags_asOBJ_LIST_PATTERN: asEObjTypeFlags = 33554432;
pub const asEObjTypeFlags_asOBJ_ENUM: asEObjTypeFlags = 67108864;
pub const asEObjTypeFlags_asOBJ_TEMPLATE_SUBTYPE: asEObjTypeFlags = 134217728;
pub const asEObjTypeFlags_asOBJ_TYPEDEF: asEObjTypeFlags = 268435456;
pub const asEObjTypeFlags_asOBJ_ABSTRACT: asEObjTypeFlags = 536870912;
pub const asEObjTypeFlags_asOBJ_APP_ALIGN16: asEObjTypeFlags = 1073741824;
pub type asEObjTypeFlags = u32;

pub const asEBehaviours_asBEHAVE_CONSTRUCT: asEBehaviours = 0;
pub const asEBehaviours_asBEHAVE_LIST_CONSTRUCT: asEBehaviours = 1;
pub const asEBehaviours_asBEHAVE_DESTRUCT: asEBehaviours = 2;
pub const asEBehaviours_asBEHAVE_FACTORY: asEBehaviours = 3;
pub const asEBehaviours_asBEHAVE_LIST_FACTORY: asEBehaviours = 4;
pub const asEBehaviours_asBEHAVE_ADDREF: asEBehaviours = 5;
pub const asEBehaviours_asBEHAVE_RELEASE: asEBehaviours = 6;
pub const asEBehaviours_asBEHAVE_GET_WEAKREF_FLAG: asEBehaviours = 7;
pub const asEBehaviours_asBEHAVE_TEMPLATE_CALLBACK: asEBehaviours = 8;
pub const asEBehaviours_asBEHAVE_FIRST_GC: asEBehaviours = 9;
pub const asEBehaviours_asBEHAVE_GETREFCOUNT: asEBehaviours = 9;
pub const asEBehaviours_asBEHAVE_SETGCFLAG: asEBehaviours = 10;
pub const asEBehaviours_asBEHAVE_GETGCFLAG: asEBehaviours = 11;
pub const asEBehaviours_asBEHAVE_ENUMREFS: asEBehaviours = 12;
pub const asEBehaviours_asBEHAVE_RELEASEREFS: asEBehaviours = 13;
pub const asEBehaviours_asBEHAVE_LAST_GC: asEBehaviours = 13;
pub const asEBehaviours_asBEHAVE_MAX: asEBehaviours = 14;
pub type asEBehaviours = u32;

pub const asEContextState_asEXECUTION_FINISHED: asEContextState = 0;
pub const asEContextState_asEXECUTION_SUSPENDED: asEContextState = 1;
pub const asEContextState_asEXECUTION_ABORTED: asEContextState = 2;
pub const asEContextState_asEXECUTION_EXCEPTION: asEContextState = 3;
pub const asEContextState_asEXECUTION_PREPARED: asEContextState = 4;
pub const asEContextState_asEXECUTION_UNINITIALIZED: asEContextState = 5;
pub const asEContextState_asEXECUTION_ACTIVE: asEContextState = 6;
pub const asEContextState_asEXECUTION_ERROR: asEContextState = 7;
pub type asEContextState = u32;

pub const asEMsgType_asMSGTYPE_ERROR: asEMsgType = 0;
pub const asEMsgType_asMSGTYPE_WARNING: asEMsgType = 1;
pub const asEMsgType_asMSGTYPE_INFORMATION: asEMsgType = 2;
pub type asEMsgType = u32;

pub const asEGCFlags_asGC_FULL_CYCLE: asEGCFlags = 1;
pub const asEGCFlags_asGC_ONE_STEP: asEGCFlags = 2;
pub const asEGCFlags_asGC_DESTROY_GARBAGE: asEGCFlags = 4;
pub const asEGCFlags_asGC_DETECT_GARBAGE: asEGCFlags = 8;
pub type asEGCFlags = u32;

pub const asETokenClass_asTC_UNKNOWN: asETokenClass = 0;
pub const asETokenClass_asTC_KEYWORD: asETokenClass = 1;
pub const asETokenClass_asTC_VALUE: asETokenClass = 2;
pub const asETokenClass_asTC_IDENTIFIER: asETokenClass = 3;
pub const asETokenClass_asTC_COMMENT: asETokenClass = 4;
pub const asETokenClass_asTC_WHITESPACE: asETokenClass = 5;
pub type asETokenClass = u32;

pub const asETypeIdFlags_asTYPEID_VOID: asETypeIdFlags = 0;
pub const asETypeIdFlags_asTYPEID_BOOL: asETypeIdFlags = 1;
pub const asETypeIdFlags_asTYPEID_INT8: asETypeIdFlags = 2;
pub const asETypeIdFlags_asTYPEID_INT16: asETypeIdFlags = 3;
pub const asETypeIdFlags_asTYPEID_INT32: asETypeIdFlags = 4;
pub const asETypeIdFlags_asTYPEID_INT64: asETypeIdFlags = 5;
pub const asETypeIdFlags_asTYPEID_UINT8: asETypeIdFlags = 6;
pub const asETypeIdFlags_asTYPEID_UINT16: asETypeIdFlags = 7;
pub const asETypeIdFlags_asTYPEID_UINT32: asETypeIdFlags = 8;
pub const asETypeIdFlags_asTYPEID_UINT64: asETypeIdFlags = 9;
pub const asETypeIdFlags_asTYPEID_FLOAT: asETypeIdFlags = 10;
pub const asETypeIdFlags_asTYPEID_DOUBLE: asETypeIdFlags = 11;
pub const asETypeIdFlags_asTYPEID_OBJHANDLE: asETypeIdFlags = 1073741824;
pub const asETypeIdFlags_asTYPEID_HANDLETOCONST: asETypeIdFlags = 536870912;
pub const asETypeIdFlags_asTYPEID_MASK_OBJECT: asETypeIdFlags = 469762048;
pub const asETypeIdFlags_asTYPEID_APPOBJECT: asETypeIdFlags = 67108864;
pub const asETypeIdFlags_asTYPEID_SCRIPTOBJECT: asETypeIdFlags = 134217728;
pub const asETypeIdFlags_asTYPEID_TEMPLATE: asETypeIdFlags = 268435456;
pub const asETypeIdFlags_asTYPEID_MASK_SEQNBR: asETypeIdFlags = 67108863;
pub type asETypeIdFlags = u32;

pub const asETypeModifiers_asTM_NONE: asETypeModifiers = 0;
pub const asETypeModifiers_asTM_INREF: asETypeModifiers = 1;
pub const asETypeModifiers_asTM_OUTREF: asETypeModifiers = 2;
pub const asETypeModifiers_asTM_INOUTREF: asETypeModifiers = 3;
pub const asETypeModifiers_asTM_CONST: asETypeModifiers = 4;
pub type asETypeModifiers = u32;

pub const asEGMFlags_asGM_ONLY_IF_EXISTS: asEGMFlags = 0;
pub const asEGMFlags_asGM_CREATE_IF_NOT_EXISTS: asEGMFlags = 1;
pub const asEGMFlags_asGM_ALWAYS_CREATE: asEGMFlags = 2;
pub type asEGMFlags = u32;

pub const asECompileFlags_asCOMP_ADD_TO_MODULE: asECompileFlags = 1;
pub type asECompileFlags = u32;

pub const asEFuncType_asFUNC_DUMMY: asEFuncType = -1;
pub const asEFuncType_asFUNC_SYSTEM: asEFuncType = 0;
pub const asEFuncType_asFUNC_SCRIPT: asEFuncType = 1;
pub const asEFuncType_asFUNC_INTERFACE: asEFuncType = 2;
pub const asEFuncType_asFUNC_VIRTUAL: asEFuncType = 3;
pub const asEFuncType_asFUNC_FUNCDEF: asEFuncType = 4;
pub const asEFuncType_asFUNC_IMPORTED: asEFuncType = 5;
pub const asEFuncType_asFUNC_DELEGATE: asEFuncType = 6;
pub type asEFuncType = i32;

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIScriptEngine {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIScriptModule {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIScriptContext {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIScriptGeneric {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIScriptObject {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asITypeInfo {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIScriptFunction {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIBinaryStream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIJITCompiler {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asIThreadManager {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asILockableSharedBool {
    _unused: [u8; 0],
}

pub type asBINARYREADFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        ptr: *mut ::std::os::raw::c_void,
        size: asUINT,
        param: *mut ::std::os::raw::c_void,
    ),
>;

pub type asBINARYWRITEFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        ptr: *const ::std::os::raw::c_void,
        size: asUINT,
        param: *mut ::std::os::raw::c_void,
    ),
>;

pub type asFUNCTION_t = ::std::option::Option<
    unsafe extern "C" fn()
>;

pub type asGENFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptGeneric
    )
>;

pub type asALLOCFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: usize
    ) -> *mut ::std::os::raw::c_void
>;

pub type asFREEFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void
    )
>;

pub type asCLEANENGINEFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptEngine
    )
>;

pub type asCLEANMODULEFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptModule
    )
>;

pub type asCLEANCONTEXTFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptContext
    )
>;

pub type asCLEANFUNCTIONFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptFunction
    )
>;

pub type asCLEANTYPEINFOFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asITypeInfo
    )
>;

pub type asCLEANSCRIPTOBJECTFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptObject
    )
>;

pub type asREQUESTCONTEXTFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptEngine,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut asIScriptContext,
>;

pub type asRETURNCONTEXTFUNC_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut asIScriptEngine,
        arg2: *mut asIScriptContext,
        arg3: *mut ::std::os::raw::c_void,
    )
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct asSMessageInfo {
    pub section: *const ::std::os::raw::c_char,
    pub row: ::std::os::raw::c_int,
    pub col: ::std::os::raw::c_int,
    pub type_: asEMsgType,
    pub message: *const ::std::os::raw::c_char,
}