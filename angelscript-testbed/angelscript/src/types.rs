use angelscript_sys::c_types::*;

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
}

#[repr(u32)]
pub enum EGMFlags {
    OnlyIfExists = asEGMFlags_asGM_ONLY_IF_EXISTS,
    CreateIfNotExists = asEGMFlags_asGM_CREATE_IF_NOT_EXISTS,
    AlwaysCreate = asEGMFlags_asGM_ALWAYS_CREATE
}