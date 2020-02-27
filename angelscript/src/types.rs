use angelscript_sys::c_types::*;

#[repr(u32)]
pub enum EScriptBool {
    True = asBOOL_asTRUE,
    False = asBOOL_asFALSE,
}

impl EScriptBool {
    pub fn from_u32(value: u32) -> EScriptBool {
        #[allow(non_upper_case_globals)]
        match value {
            asBOOL_asTRUE => EScriptBool::True,
            asBOOL_asFALSE => EScriptBool::False,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(i32)]
pub enum EReturnCodes {
    Success = asERetCodes_asSUCCESS,
    Error = asERetCodes_asERROR,
    ContextActive = asERetCodes_asCONTEXT_ACTIVE,
    ContextNotFinished = asERetCodes_asCONTEXT_NOT_FINISHED,
    ContextNotPrepared = asERetCodes_asCONTEXT_NOT_PREPARED,
    InvalidArg = asERetCodes_asINVALID_ARG,
    NoFunction = asERetCodes_asNO_FUNCTION,
    NotSupported = asERetCodes_asNOT_SUPPORTED,
    InvalidName = asERetCodes_asINVALID_NAME,
    NameTaken = asERetCodes_asNAME_TAKEN,
    InvalidDeclaration = asERetCodes_asINVALID_DECLARATION,
    InvalidObject = asERetCodes_asINVALID_OBJECT,
    InvalidType = asERetCodes_asINVALID_TYPE,
    AlreadyRegistered = asERetCodes_asALREADY_REGISTERED,
    MultipleFunctions = asERetCodes_asMULTIPLE_FUNCTIONS,
    NoModule = asERetCodes_asNO_MODULE,
    NoGlobalVar = asERetCodes_asNO_GLOBAL_VAR,
    InvalidConfiguration = asERetCodes_asINVALID_CONFIGURATION,
    InvalidInterface = asERetCodes_asINVALID_INTERFACE,
    CantBindAllFunctions = asERetCodes_asCANT_BIND_ALL_FUNCTIONS,
    LowerArrayDimensionNotRegistered = asERetCodes_asLOWER_ARRAY_DIMENSION_NOT_REGISTERED,
    WrongConfigGroup = asERetCodes_asWRONG_CONFIG_GROUP,
    ConfigGroupIsInUse = asERetCodes_asCONFIG_GROUP_IS_IN_USE,
    IllegalBehaviourForType = asERetCodes_asILLEGAL_BEHAVIOUR_FOR_TYPE,
    WrongCallingConv = asERetCodes_asWRONG_CALLING_CONV,
    BuildInProgress = asERetCodes_asBUILD_IN_PROGRESS,
    InitGlobalVarsFailed = asERetCodes_asINIT_GLOBAL_VARS_FAILED,
    OutOfMemory = asERetCodes_asOUT_OF_MEMORY,
    ModuleIsInUse = asERetCodes_asMODULE_IS_IN_USE
}

impl EReturnCodes {
    pub fn from_i32(value: i32) -> EReturnCodes {
        #[allow(non_upper_case_globals)]
        match value {
            asERetCodes_asSUCCESS => EReturnCodes::Success,
            asERetCodes_asERROR => EReturnCodes::Error,
            asERetCodes_asCONTEXT_ACTIVE => EReturnCodes::ContextActive,
            asERetCodes_asCONTEXT_NOT_FINISHED => EReturnCodes::ContextNotFinished,
            asERetCodes_asCONTEXT_NOT_PREPARED => EReturnCodes::ContextNotPrepared,
            asERetCodes_asINVALID_ARG => EReturnCodes::InvalidArg,
            asERetCodes_asNO_FUNCTION => EReturnCodes::NoFunction,
            asERetCodes_asNOT_SUPPORTED => EReturnCodes::NotSupported,
            asERetCodes_asINVALID_NAME => EReturnCodes::InvalidName,
            asERetCodes_asNAME_TAKEN => EReturnCodes::NameTaken,
            asERetCodes_asINVALID_DECLARATION => EReturnCodes::InvalidDeclaration,
            asERetCodes_asINVALID_OBJECT => EReturnCodes::InvalidObject,
            asERetCodes_asINVALID_TYPE => EReturnCodes::InvalidType,
            asERetCodes_asALREADY_REGISTERED => EReturnCodes::AlreadyRegistered,
            asERetCodes_asMULTIPLE_FUNCTIONS => EReturnCodes::MultipleFunctions,
            asERetCodes_asNO_MODULE => EReturnCodes::NoModule,
            asERetCodes_asNO_GLOBAL_VAR => EReturnCodes::NoGlobalVar,
            asERetCodes_asINVALID_CONFIGURATION => EReturnCodes::InvalidConfiguration,
            asERetCodes_asINVALID_INTERFACE => EReturnCodes::InvalidInterface,
            asERetCodes_asCANT_BIND_ALL_FUNCTIONS => EReturnCodes::CantBindAllFunctions,
            asERetCodes_asLOWER_ARRAY_DIMENSION_NOT_REGISTERED => EReturnCodes::LowerArrayDimensionNotRegistered,
            asERetCodes_asWRONG_CONFIG_GROUP => EReturnCodes::WrongConfigGroup,
            asERetCodes_asCONFIG_GROUP_IS_IN_USE => EReturnCodes::ConfigGroupIsInUse,
            asERetCodes_asILLEGAL_BEHAVIOUR_FOR_TYPE => EReturnCodes::IllegalBehaviourForType,
            asERetCodes_asWRONG_CALLING_CONV => EReturnCodes::WrongCallingConv,
            asERetCodes_asBUILD_IN_PROGRESS => EReturnCodes::BuildInProgress,
            asERetCodes_asINIT_GLOBAL_VARS_FAILED => EReturnCodes::InitGlobalVarsFailed,
            asERetCodes_asOUT_OF_MEMORY => EReturnCodes::OutOfMemory,
            asERetCodes_asMODULE_IS_IN_USE => EReturnCodes::ModuleIsInUse,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_i32(self) -> i32 {
        return self as i32;
    }
}


#[repr(u32)]
pub enum EEngineProp {
    AllowUnsafeReferences = asEEngineProp_asEP_ALLOW_UNSAFE_REFERENCES,
    OptimizeBytecode = asEEngineProp_asEP_OPTIMIZE_BYTECODE,
    CopyScriptSections = asEEngineProp_asEP_COPY_SCRIPT_SECTIONS,
    MaxStackSize = asEEngineProp_asEP_MAX_STACK_SIZE,
    UseCharacterLiterals = asEEngineProp_asEP_USE_CHARACTER_LITERALS,
    AllowMultilineStrings = asEEngineProp_asEP_ALLOW_MULTILINE_STRINGS,
    AllowImplicitHandleTypes = asEEngineProp_asEP_ALLOW_IMPLICIT_HANDLE_TYPES,
    BuildWithoutLineCues = asEEngineProp_asEP_BUILD_WITHOUT_LINE_CUES,
    InitGlobalVarsAfterBuild = asEEngineProp_asEP_INIT_GLOBAL_VARS_AFTER_BUILD,
    RequireEnumScope = asEEngineProp_asEP_REQUIRE_ENUM_SCOPE,
    ScriptScanner = asEEngineProp_asEP_SCRIPT_SCANNER,
    IncludeJitInstructions = asEEngineProp_asEP_INCLUDE_JIT_INSTRUCTIONS,
    StringEncoding = asEEngineProp_asEP_STRING_ENCODING,
    PropertyAccessorMode = asEEngineProp_asEP_PROPERTY_ACCESSOR_MODE,
    ExpandDefArrayToImpl = asEEngineProp_asEP_EXPAND_DEF_ARRAY_TO_TMPL,
    AutoGarbageCollect = asEEngineProp_asEP_AUTO_GARBAGE_COLLECT,
    DisallowGlobalVars = asEEngineProp_asEP_DISALLOW_GLOBAL_VARS,
    AlwaysImplDefaultConstruct = asEEngineProp_asEP_ALWAYS_IMPL_DEFAULT_CONSTRUCT,
    CompilerWarnings = asEEngineProp_asEP_COMPILER_WARNINGS,
    DisallowValueAssignForRefType = asEEngineProp_asEP_DISALLOW_VALUE_ASSIGN_FOR_REF_TYPE,
    AlterSyntaxNamedArgs = asEEngineProp_asEP_ALTER_SYNTAX_NAMED_ARGS,
    DisableIntegerDivision = asEEngineProp_asEP_DISABLE_INTEGER_DIVISION,
    DisallowEmptyListElements = asEEngineProp_asEP_DISALLOW_EMPTY_LIST_ELEMENTS,
    PrivatePropAsProtected = asEEngineProp_asEP_PRIVATE_PROP_AS_PROTECTED,
    AllowUnicodeIdentifiers = asEEngineProp_asEP_ALLOW_UNICODE_IDENTIFIERS,
    HeredocTrimCode = asEEngineProp_asEP_HEREDOC_TRIM_MODE,
    LastProperty = asEEngineProp_asEP_LAST_PROPERTY
}

impl EEngineProp {
    pub fn from_u32(value: u32) -> EEngineProp {
        #[allow(non_upper_case_globals)]
        match value {
            asEEngineProp_asEP_ALLOW_UNSAFE_REFERENCES => EEngineProp::AllowUnsafeReferences,
            asEEngineProp_asEP_OPTIMIZE_BYTECODE => EEngineProp::OptimizeBytecode,
            asEEngineProp_asEP_COPY_SCRIPT_SECTIONS => EEngineProp::CopyScriptSections,
            asEEngineProp_asEP_MAX_STACK_SIZE => EEngineProp::MaxStackSize,
            asEEngineProp_asEP_USE_CHARACTER_LITERALS => EEngineProp::UseCharacterLiterals,
            asEEngineProp_asEP_ALLOW_MULTILINE_STRINGS => EEngineProp::AllowMultilineStrings,
            asEEngineProp_asEP_ALLOW_IMPLICIT_HANDLE_TYPES => EEngineProp::AllowImplicitHandleTypes,
            asEEngineProp_asEP_BUILD_WITHOUT_LINE_CUES => EEngineProp::BuildWithoutLineCues,
            asEEngineProp_asEP_INIT_GLOBAL_VARS_AFTER_BUILD => EEngineProp::InitGlobalVarsAfterBuild,
            asEEngineProp_asEP_REQUIRE_ENUM_SCOPE => EEngineProp::RequireEnumScope,
            asEEngineProp_asEP_SCRIPT_SCANNER => EEngineProp::ScriptScanner,
            asEEngineProp_asEP_INCLUDE_JIT_INSTRUCTIONS => EEngineProp::IncludeJitInstructions,
            asEEngineProp_asEP_STRING_ENCODING => EEngineProp::StringEncoding,
            asEEngineProp_asEP_PROPERTY_ACCESSOR_MODE => EEngineProp::PropertyAccessorMode,
            asEEngineProp_asEP_EXPAND_DEF_ARRAY_TO_TMPL => EEngineProp::ExpandDefArrayToImpl,
            asEEngineProp_asEP_AUTO_GARBAGE_COLLECT => EEngineProp::AutoGarbageCollect,
            asEEngineProp_asEP_DISALLOW_GLOBAL_VARS => EEngineProp::DisallowGlobalVars,
            asEEngineProp_asEP_ALWAYS_IMPL_DEFAULT_CONSTRUCT => EEngineProp::AlwaysImplDefaultConstruct,
            asEEngineProp_asEP_COMPILER_WARNINGS => EEngineProp::CompilerWarnings,
            asEEngineProp_asEP_DISALLOW_VALUE_ASSIGN_FOR_REF_TYPE => EEngineProp::DisallowValueAssignForRefType,
            asEEngineProp_asEP_ALTER_SYNTAX_NAMED_ARGS => EEngineProp::AlterSyntaxNamedArgs,
            asEEngineProp_asEP_DISABLE_INTEGER_DIVISION => EEngineProp::DisableIntegerDivision,
            asEEngineProp_asEP_DISALLOW_EMPTY_LIST_ELEMENTS => EEngineProp::DisallowEmptyListElements,
            asEEngineProp_asEP_PRIVATE_PROP_AS_PROTECTED => EEngineProp::PrivatePropAsProtected,
            asEEngineProp_asEP_ALLOW_UNICODE_IDENTIFIERS => EEngineProp::AllowUnicodeIdentifiers,
            asEEngineProp_asEP_HEREDOC_TRIM_MODE => EEngineProp::HeredocTrimCode,
            asEEngineProp_asEP_LAST_PROPERTY => EEngineProp::LastProperty,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum ECallConvTypes {
    CDecl = asECallConvTypes_asCALL_CDECL,
    Std = asECallConvTypes_asCALL_STDCALL,
    ThisAsGlobal = asECallConvTypes_asCALL_THISCALL_ASGLOBAL,
    This = asECallConvTypes_asCALL_THISCALL,
    CDeclObjLast = asECallConvTypes_asCALL_CDECL_OBJLAST,
    CDeclObjFirst = asECallConvTypes_asCALL_CDECL_OBJFIRST,
    Generic = asECallConvTypes_asCALL_GENERIC,
    ThisObjLast = asECallConvTypes_asCALL_THISCALL_OBJLAST,
    ThisObjFirst = asECallConvTypes_asCALL_THISCALL_OBJFIRST,
}

impl ECallConvTypes {
    pub fn from_u32(value: u32) -> ECallConvTypes {
        #[allow(non_upper_case_globals)]
        match value {
            asECallConvTypes_asCALL_CDECL => ECallConvTypes::CDecl,
            asECallConvTypes_asCALL_STDCALL => ECallConvTypes::Std,
            asECallConvTypes_asCALL_THISCALL_ASGLOBAL => ECallConvTypes::ThisAsGlobal,
            asECallConvTypes_asCALL_THISCALL => ECallConvTypes::This,
            asECallConvTypes_asCALL_CDECL_OBJLAST => ECallConvTypes::CDeclObjLast,
            asECallConvTypes_asCALL_CDECL_OBJFIRST => ECallConvTypes::CDeclObjFirst,
            asECallConvTypes_asCALL_GENERIC => ECallConvTypes::Generic,
            asECallConvTypes_asCALL_THISCALL_OBJLAST => ECallConvTypes::ThisObjLast,
            asECallConvTypes_asCALL_THISCALL_OBJFIRST => ECallConvTypes::ThisObjFirst,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum EObjTypeFlags {
    Ref = asEObjTypeFlags_asOBJ_REF,
    Value = asEObjTypeFlags_asOBJ_VALUE,
    GC = asEObjTypeFlags_asOBJ_GC,
    POD = asEObjTypeFlags_asOBJ_POD,
    NoHandle = asEObjTypeFlags_asOBJ_NOHANDLE,
    Scoped = asEObjTypeFlags_asOBJ_SCOPED,
    Template = asEObjTypeFlags_asOBJ_TEMPLATE,
    AsHandle = asEObjTypeFlags_asOBJ_ASHANDLE,
    AppClass = asEObjTypeFlags_asOBJ_APP_CLASS,
    AppClassConstructor = asEObjTypeFlags_asOBJ_APP_CLASS_CONSTRUCTOR,
    AppClassDestructor = asEObjTypeFlags_asOBJ_APP_CLASS_DESTRUCTOR,
    AppClassAssignment = asEObjTypeFlags_asOBJ_APP_CLASS_ASSIGNMENT,
    AppClassCopyConstructor = asEObjTypeFlags_asOBJ_APP_CLASS_COPY_CONSTRUCTOR,
    AppClassC = asEObjTypeFlags_asOBJ_APP_CLASS_C,
    AppClassCD = asEObjTypeFlags_asOBJ_APP_CLASS_CD,
    AppClassCA = asEObjTypeFlags_asOBJ_APP_CLASS_CA,
    AppClassCK = asEObjTypeFlags_asOBJ_APP_CLASS_CK,
    AppClassCDA = asEObjTypeFlags_asOBJ_APP_CLASS_CDA,
    AppClassCDK = asEObjTypeFlags_asOBJ_APP_CLASS_CDK,
    AppClassCAK = asEObjTypeFlags_asOBJ_APP_CLASS_CAK,
    AppClassCDAK = asEObjTypeFlags_asOBJ_APP_CLASS_CDAK,
    AppClassD = asEObjTypeFlags_asOBJ_APP_CLASS_D,
    AppClassDA = asEObjTypeFlags_asOBJ_APP_CLASS_DA,
    AppClassDK = asEObjTypeFlags_asOBJ_APP_CLASS_DK,
    AppClassDAK = asEObjTypeFlags_asOBJ_APP_CLASS_DAK,
    AppClassA = asEObjTypeFlags_asOBJ_APP_CLASS_A,
    AppClassAK = asEObjTypeFlags_asOBJ_APP_CLASS_AK,
    AppClassK = asEObjTypeFlags_asOBJ_APP_CLASS_K,
    AppPrimitive = asEObjTypeFlags_asOBJ_APP_PRIMITIVE,
    AppFloat = asEObjTypeFlags_asOBJ_APP_FLOAT,
    AppArray = asEObjTypeFlags_asOBJ_APP_ARRAY,
    AppClassAllInts = asEObjTypeFlags_asOBJ_APP_CLASS_ALLINTS,
    AppClassAllFloats = asEObjTypeFlags_asOBJ_APP_CLASS_ALLFLOATS,
    NoCount = asEObjTypeFlags_asOBJ_NOCOUNT,
    AppClassAlign8 = asEObjTypeFlags_asOBJ_APP_CLASS_ALIGN8,
    ImplicitHandle = asEObjTypeFlags_asOBJ_IMPLICIT_HANDLE,
    MaskValidFlags = asEObjTypeFlags_asOBJ_MASK_VALID_FLAGS,
    ScriptObject = asEObjTypeFlags_asOBJ_SCRIPT_OBJECT,
    Shared = asEObjTypeFlags_asOBJ_SHARED,
    NoInherit = asEObjTypeFlags_asOBJ_NOINHERIT,
    FuncDef = asEObjTypeFlags_asOBJ_FUNCDEF,
    ListPattern = asEObjTypeFlags_asOBJ_LIST_PATTERN,
    Enum = asEObjTypeFlags_asOBJ_ENUM,
    TemplateSubtype = asEObjTypeFlags_asOBJ_TEMPLATE_SUBTYPE,
    Typedef= asEObjTypeFlags_asOBJ_TYPEDEF,
    Abstract = asEObjTypeFlags_asOBJ_ABSTRACT,
    AppAlign16 = asEObjTypeFlags_asOBJ_APP_ALIGN16,
}

impl EObjTypeFlags {
    pub fn from_u32(value: u32) -> EObjTypeFlags {
        #[allow(non_upper_case_globals)]
        match value {
            asEObjTypeFlags_asOBJ_REF => EObjTypeFlags::Ref,
            asEObjTypeFlags_asOBJ_VALUE => EObjTypeFlags::Value,
            asEObjTypeFlags_asOBJ_GC => EObjTypeFlags::GC,
            asEObjTypeFlags_asOBJ_POD => EObjTypeFlags::POD,
            asEObjTypeFlags_asOBJ_NOHANDLE => EObjTypeFlags::NoHandle,
            asEObjTypeFlags_asOBJ_SCOPED => EObjTypeFlags::Scoped,
            asEObjTypeFlags_asOBJ_TEMPLATE => EObjTypeFlags::Template,
            asEObjTypeFlags_asOBJ_ASHANDLE => EObjTypeFlags::AsHandle,
            asEObjTypeFlags_asOBJ_APP_CLASS => EObjTypeFlags::AppClass,
            asEObjTypeFlags_asOBJ_APP_CLASS_CONSTRUCTOR => EObjTypeFlags::AppClassConstructor,
            asEObjTypeFlags_asOBJ_APP_CLASS_DESTRUCTOR => EObjTypeFlags::AppClassDestructor,
            asEObjTypeFlags_asOBJ_APP_CLASS_ASSIGNMENT => EObjTypeFlags::AppClassAssignment,
            asEObjTypeFlags_asOBJ_APP_CLASS_COPY_CONSTRUCTOR => EObjTypeFlags::AppClassCopyConstructor,
            asEObjTypeFlags_asOBJ_APP_CLASS_C => EObjTypeFlags::AppClassC,
            asEObjTypeFlags_asOBJ_APP_CLASS_CD => EObjTypeFlags::AppClassCD,
            asEObjTypeFlags_asOBJ_APP_CLASS_CA => EObjTypeFlags::AppClassCA,
            asEObjTypeFlags_asOBJ_APP_CLASS_CK => EObjTypeFlags::AppClassCK,
            asEObjTypeFlags_asOBJ_APP_CLASS_CDA => EObjTypeFlags::AppClassCDA,
            asEObjTypeFlags_asOBJ_APP_CLASS_CDK => EObjTypeFlags::AppClassCDK,
            asEObjTypeFlags_asOBJ_APP_CLASS_CAK => EObjTypeFlags::AppClassCAK,
            asEObjTypeFlags_asOBJ_APP_CLASS_CDAK => EObjTypeFlags::AppClassCDAK,
            asEObjTypeFlags_asOBJ_APP_CLASS_D => EObjTypeFlags::AppClassD,
            asEObjTypeFlags_asOBJ_APP_CLASS_DA => EObjTypeFlags::AppClassDA,
            asEObjTypeFlags_asOBJ_APP_CLASS_DK => EObjTypeFlags::AppClassDK,
            asEObjTypeFlags_asOBJ_APP_CLASS_DAK => EObjTypeFlags::AppClassDAK,
            asEObjTypeFlags_asOBJ_APP_CLASS_A => EObjTypeFlags::AppClassA,
            asEObjTypeFlags_asOBJ_APP_CLASS_AK => EObjTypeFlags::AppClassAK,
            asEObjTypeFlags_asOBJ_APP_CLASS_K => EObjTypeFlags::AppClassK,
            asEObjTypeFlags_asOBJ_APP_PRIMITIVE => EObjTypeFlags::AppPrimitive,
            asEObjTypeFlags_asOBJ_APP_FLOAT => EObjTypeFlags::AppFloat,
            asEObjTypeFlags_asOBJ_APP_ARRAY => EObjTypeFlags::AppArray,
            asEObjTypeFlags_asOBJ_APP_CLASS_ALLINTS => EObjTypeFlags::AppClassAllInts,
            asEObjTypeFlags_asOBJ_APP_CLASS_ALLFLOATS => EObjTypeFlags::AppClassAllFloats,
            asEObjTypeFlags_asOBJ_NOCOUNT => EObjTypeFlags::NoCount,
            asEObjTypeFlags_asOBJ_APP_CLASS_ALIGN8 => EObjTypeFlags::AppClassAlign8,
            asEObjTypeFlags_asOBJ_IMPLICIT_HANDLE => EObjTypeFlags::ImplicitHandle,
            asEObjTypeFlags_asOBJ_MASK_VALID_FLAGS => EObjTypeFlags::MaskValidFlags,
            asEObjTypeFlags_asOBJ_SCRIPT_OBJECT => EObjTypeFlags::ScriptObject,
            asEObjTypeFlags_asOBJ_SHARED => EObjTypeFlags::Shared,
            asEObjTypeFlags_asOBJ_NOINHERIT => EObjTypeFlags::NoInherit,
            asEObjTypeFlags_asOBJ_FUNCDEF => EObjTypeFlags::FuncDef,
            asEObjTypeFlags_asOBJ_LIST_PATTERN => EObjTypeFlags::ListPattern,
            asEObjTypeFlags_asOBJ_ENUM => EObjTypeFlags::Enum,
            asEObjTypeFlags_asOBJ_TEMPLATE_SUBTYPE => EObjTypeFlags::TemplateSubtype,
            asEObjTypeFlags_asOBJ_TYPEDEF => EObjTypeFlags::Typedef,
            asEObjTypeFlags_asOBJ_ABSTRACT => EObjTypeFlags::Abstract,
            asEObjTypeFlags_asOBJ_APP_ALIGN16 => EObjTypeFlags::AppAlign16,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum EBehaviours {
    Construct = asEBehaviours_asBEHAVE_CONSTRUCT,
    ListConstruct = asEBehaviours_asBEHAVE_LIST_CONSTRUCT,
    Destruct = asEBehaviours_asBEHAVE_DESTRUCT,
    Factory = asEBehaviours_asBEHAVE_FACTORY,
    ListFactory = asEBehaviours_asBEHAVE_LIST_FACTORY,
    AddRef = asEBehaviours_asBEHAVE_ADDREF,
    Release = asEBehaviours_asBEHAVE_RELEASE,
    GetWeakRefFlag= asEBehaviours_asBEHAVE_GET_WEAKREF_FLAG,
    TemplateCallback = asEBehaviours_asBEHAVE_TEMPLATE_CALLBACK,
    //FirstGC = asEBehaviours_asBEHAVE_FIRST_GC,
    GetRefCount = asEBehaviours_asBEHAVE_GETREFCOUNT,
    SetGCFlag = asEBehaviours_asBEHAVE_SETGCFLAG,
    GetGCFlag = asEBehaviours_asBEHAVE_GETGCFLAG,
    EnumRefs = asEBehaviours_asBEHAVE_ENUMREFS,
    ReleaseRefs = asEBehaviours_asBEHAVE_RELEASEREFS,
    //LastGC = asEBehaviours_asBEHAVE_LAST_GC,
    BehaveMax = asEBehaviours_asBEHAVE_MAX,
}

impl EBehaviours {
    pub fn from_u32(value: u32) -> EBehaviours {
        #[allow(non_upper_case_globals)]
        match value {
            asEBehaviours_asBEHAVE_CONSTRUCT => EBehaviours::Construct,
            asEBehaviours_asBEHAVE_LIST_CONSTRUCT => EBehaviours::ListConstruct,
            asEBehaviours_asBEHAVE_DESTRUCT => EBehaviours::Destruct,
            asEBehaviours_asBEHAVE_FACTORY => EBehaviours::Factory,
            asEBehaviours_asBEHAVE_LIST_FACTORY => EBehaviours::ListFactory,
            asEBehaviours_asBEHAVE_ADDREF => EBehaviours::AddRef,
            asEBehaviours_asBEHAVE_RELEASE => EBehaviours::Release,
            asEBehaviours_asBEHAVE_GET_WEAKREF_FLAG => EBehaviours::GetWeakRefFlag,
            asEBehaviours_asBEHAVE_TEMPLATE_CALLBACK => EBehaviours::TemplateCallback,
            //asEBehaviours_asBEHAVE_FIRST_GC => EBehaviours::FirstGC,
            asEBehaviours_asBEHAVE_GETREFCOUNT => EBehaviours::GetRefCount,
            asEBehaviours_asBEHAVE_SETGCFLAG => EBehaviours::SetGCFlag,
            asEBehaviours_asBEHAVE_GETGCFLAG => EBehaviours::GetGCFlag,
            asEBehaviours_asBEHAVE_ENUMREFS => EBehaviours::EnumRefs,
            asEBehaviours_asBEHAVE_RELEASEREFS => EBehaviours::ReleaseRefs,
            //asEBehaviours_asBEHAVE_LAST_GC => EBehaviours::LastGC,
            asEBehaviours_asBEHAVE_MAX => EBehaviours::BehaveMax,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum EContextState {
    ExecutionFinished = asEContextState_asEXECUTION_FINISHED,
    ExecutionSuspended = asEContextState_asEXECUTION_SUSPENDED,
    ExecutionAborted = asEContextState_asEXECUTION_ABORTED,
    ExecutionException = asEContextState_asEXECUTION_EXCEPTION,
    ExecutionPrepared = asEContextState_asEXECUTION_PREPARED,
    ExecutionUninitialized = asEContextState_asEXECUTION_UNINITIALIZED,
    ExecutionActive = asEContextState_asEXECUTION_ACTIVE,
    ExecutionError = asEContextState_asEXECUTION_ERROR,
}

impl EContextState {
    pub fn from_u32(value: u32) -> EContextState {
        #[allow(non_upper_case_globals)]
        match value {
            asEContextState_asEXECUTION_FINISHED => EContextState::ExecutionFinished,
            asEContextState_asEXECUTION_SUSPENDED => EContextState::ExecutionSuspended,
            asEContextState_asEXECUTION_ABORTED => EContextState::ExecutionAborted,
            asEContextState_asEXECUTION_EXCEPTION => EContextState::ExecutionException,
            asEContextState_asEXECUTION_PREPARED => EContextState::ExecutionPrepared,
            asEContextState_asEXECUTION_UNINITIALIZED => EContextState::ExecutionUninitialized,
            asEContextState_asEXECUTION_ACTIVE => EContextState::ExecutionActive,
            asEContextState_asEXECUTION_ERROR => EContextState::ExecutionError,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum EMsgType {
    Error = asEMsgType_asMSGTYPE_ERROR,
    Warning = asEMsgType_asMSGTYPE_WARNING,
    Information = asEMsgType_asMSGTYPE_INFORMATION,
}

impl EMsgType {
    pub fn from_u32(value: u32) -> EMsgType {
        #[allow(non_upper_case_globals)]
        match value {
            asEMsgType_asMSGTYPE_ERROR => EMsgType::Error,
            asEMsgType_asMSGTYPE_WARNING => EMsgType::Warning,
            asEMsgType_asMSGTYPE_INFORMATION => EMsgType::Information,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum EGCFlags {
    FullCycle = asEGCFlags_asGC_FULL_CYCLE,
    OneStep = asEGCFlags_asGC_ONE_STEP,
    DestroyGarbage = asEGCFlags_asGC_DESTROY_GARBAGE,
    DetectGarbage = asEGCFlags_asGC_DETECT_GARBAGE,
}

impl EGCFlags {
    pub fn from_u32(value: u32) -> EGCFlags {
        #[allow(non_upper_case_globals)]
        match value {
            asEGCFlags_asGC_FULL_CYCLE => EGCFlags::FullCycle,
            asEGCFlags_asGC_ONE_STEP => EGCFlags::OneStep,
            asEGCFlags_asGC_DESTROY_GARBAGE => EGCFlags::DestroyGarbage,
            asEGCFlags_asGC_DETECT_GARBAGE => EGCFlags::DetectGarbage,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum ETokenClass {
    Unknown = asETokenClass_asTC_UNKNOWN,
    Keyword = asETokenClass_asTC_KEYWORD,
    Value = asETokenClass_asTC_VALUE,
    Identifier = asETokenClass_asTC_IDENTIFIER,
    Comment = asETokenClass_asTC_COMMENT,
    Whitespace = asETokenClass_asTC_WHITESPACE,
}

impl ETokenClass {
    pub fn from_u32(value: u32) -> ETokenClass {
        #[allow(non_upper_case_globals)]
        match value {
            asETokenClass_asTC_UNKNOWN => ETokenClass::Unknown,
            asETokenClass_asTC_KEYWORD => ETokenClass::Keyword,
            asETokenClass_asTC_VALUE => ETokenClass::Value,
            asETokenClass_asTC_IDENTIFIER => ETokenClass::Identifier,
            asETokenClass_asTC_COMMENT => ETokenClass::Comment,
            asETokenClass_asTC_WHITESPACE => ETokenClass::Whitespace,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum ETypeIDFlags {
    Void = asETypeIdFlags_asTYPEID_VOID,
    Bool = asETypeIdFlags_asTYPEID_BOOL,
    Int8 = asETypeIdFlags_asTYPEID_INT8,
    Int16 = asETypeIdFlags_asTYPEID_INT16,
    Int32 = asETypeIdFlags_asTYPEID_INT32,
    Int64 = asETypeIdFlags_asTYPEID_INT64,
    UInt8 = asETypeIdFlags_asTYPEID_UINT8,
    UInt16 = asETypeIdFlags_asTYPEID_UINT16,
    UInt32 = asETypeIdFlags_asTYPEID_UINT32,
    UInt64 = asETypeIdFlags_asTYPEID_UINT64,
    Float = asETypeIdFlags_asTYPEID_FLOAT,
    Double = asETypeIdFlags_asTYPEID_DOUBLE,
    ObjHandle = asETypeIdFlags_asTYPEID_OBJHANDLE,
    HandleToConst = asETypeIdFlags_asTYPEID_HANDLETOCONST,
    MaskObject = asETypeIdFlags_asTYPEID_MASK_OBJECT,
    AppObject = asETypeIdFlags_asTYPEID_APPOBJECT,
    ScriptObject = asETypeIdFlags_asTYPEID_SCRIPTOBJECT,
    Template = asETypeIdFlags_asTYPEID_TEMPLATE,
    MaskSeqNBR = asETypeIdFlags_asTYPEID_MASK_SEQNBR,
}

impl ETypeIDFlags {
    pub fn from_u32(value: u32) -> ETypeIDFlags {
        #[allow(non_upper_case_globals)]
        match value {
            asETypeIdFlags_asTYPEID_VOID => ETypeIDFlags::Void,
            asETypeIdFlags_asTYPEID_BOOL => ETypeIDFlags::Bool,
            asETypeIdFlags_asTYPEID_INT8 => ETypeIDFlags::Int8,
            asETypeIdFlags_asTYPEID_INT16 => ETypeIDFlags::Int16,
            asETypeIdFlags_asTYPEID_INT32 => ETypeIDFlags::Int32,
            asETypeIdFlags_asTYPEID_INT64 => ETypeIDFlags::Int64,
            asETypeIdFlags_asTYPEID_UINT8 => ETypeIDFlags::UInt8,
            asETypeIdFlags_asTYPEID_UINT16 => ETypeIDFlags::UInt16,
            asETypeIdFlags_asTYPEID_UINT32 => ETypeIDFlags::UInt32,
            asETypeIdFlags_asTYPEID_UINT64 => ETypeIDFlags::UInt64,
            asETypeIdFlags_asTYPEID_FLOAT => ETypeIDFlags::Float,
            asETypeIdFlags_asTYPEID_DOUBLE => ETypeIDFlags::Double,
            asETypeIdFlags_asTYPEID_OBJHANDLE => ETypeIDFlags::ObjHandle,
            asETypeIdFlags_asTYPEID_HANDLETOCONST => ETypeIDFlags::HandleToConst,
            asETypeIdFlags_asTYPEID_MASK_OBJECT => ETypeIDFlags::MaskObject,
            asETypeIdFlags_asTYPEID_APPOBJECT => ETypeIDFlags::AppObject,
            asETypeIdFlags_asTYPEID_SCRIPTOBJECT => ETypeIDFlags::ScriptObject,
            asETypeIdFlags_asTYPEID_TEMPLATE => ETypeIDFlags::Template,
            asETypeIdFlags_asTYPEID_MASK_SEQNBR => ETypeIDFlags::MaskSeqNBR,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum ETypeModifiers {
    None = asETypeModifiers_asTM_NONE,
    InRef= asETypeModifiers_asTM_INREF,
    OutRef = asETypeModifiers_asTM_OUTREF,
    InOutRef = asETypeModifiers_asTM_INOUTREF,
    Const = asETypeModifiers_asTM_CONST,
}

impl ETypeModifiers {
    pub fn from_u32(value: u32) -> ETypeModifiers {
        #[allow(non_upper_case_globals)]
        match value {
            asETypeModifiers_asTM_NONE => ETypeModifiers::None,
            asETypeModifiers_asTM_INREF => ETypeModifiers::InRef,
            asETypeModifiers_asTM_OUTREF => ETypeModifiers::OutRef,
            asETypeModifiers_asTM_INOUTREF => ETypeModifiers::InOutRef,
            asETypeModifiers_asTM_CONST => ETypeModifiers::Const,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum EGMFlags {
    OnlyIfExists = asEGMFlags_asGM_ONLY_IF_EXISTS,
    CreateIfNotExists = asEGMFlags_asGM_CREATE_IF_NOT_EXISTS,
    AlwaysCreate = asEGMFlags_asGM_ALWAYS_CREATE
}

impl EGMFlags {
    pub fn from_u32(value: u32) -> EGMFlags {
        #[allow(non_upper_case_globals)]
        match value {
            asEGMFlags_asGM_ONLY_IF_EXISTS => EGMFlags::OnlyIfExists,
            asEGMFlags_asGM_CREATE_IF_NOT_EXISTS => EGMFlags::CreateIfNotExists,
            asEGMFlags_asGM_ALWAYS_CREATE => EGMFlags::AlwaysCreate,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(u32)]
pub enum ECompileFlags {
    AddToModule = asECompileFlags_asCOMP_ADD_TO_MODULE,
}

impl ECompileFlags {
    pub fn from_u32(value: u32) -> ECompileFlags {
        #[allow(non_upper_case_globals)]
        match value {
            asECompileFlags_asCOMP_ADD_TO_MODULE => ECompileFlags::AddToModule,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> u32 {
        return self as u32;
    }
}


#[repr(i32)]
pub enum EFuncType {
    Dummy = asEFuncType_asFUNC_DUMMY,
    System = asEFuncType_asFUNC_SYSTEM,
    Script = asEFuncType_asFUNC_SCRIPT,
    Interface = asEFuncType_asFUNC_INTERFACE,
    Virtual = asEFuncType_asFUNC_VIRTUAL,
    FuncDef= asEFuncType_asFUNC_FUNCDEF,
    Imported = asEFuncType_asFUNC_IMPORTED,
    Delegate = asEFuncType_asFUNC_DELEGATE,
}

impl EFuncType {
    pub fn from_i32(value: i32) -> EFuncType {
        #[allow(non_upper_case_globals)]
        match value {
            asEFuncType_asFUNC_DUMMY => EFuncType::Dummy,
            asEFuncType_asFUNC_SYSTEM => EFuncType::System,
            asEFuncType_asFUNC_SCRIPT => EFuncType::Script,
            asEFuncType_asFUNC_INTERFACE => EFuncType::Interface,
            asEFuncType_asFUNC_VIRTUAL => EFuncType::Virtual,
            asEFuncType_asFUNC_FUNCDEF => EFuncType::FuncDef,
            asEFuncType_asFUNC_IMPORTED => EFuncType::Imported,
            asEFuncType_asFUNC_DELEGATE => EFuncType::Delegate,
            _ => panic!("unknown value: {}", value)
        }
    }

    pub fn to_u32(self) -> i32 {
        return self as i32;
    }
}

pub struct FuncBindInfo {
    pub func: asFUNCTION_t,
    pub as_decl: &'static str,
}