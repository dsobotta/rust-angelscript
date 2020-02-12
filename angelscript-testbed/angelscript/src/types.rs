use angelscript_sys::c_types::*;

#[repr(u32)]
pub enum EGMFlags {
    OnlyIfExists = asEGMFlags_asGM_ONLY_IF_EXISTS,
    CreateIfNotExists = asEGMFlags_asGM_CREATE_IF_NOT_EXISTS,
    AlwaysCreate = asEGMFlags_asGM_ALWAYS_CREATE
}