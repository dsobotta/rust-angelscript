pub mod angelscript;

use angelscript::read_cstring;
use angelscript::types::*;

unsafe extern "C" fn callback(msgPtr: *const asSMessageInfo, params: *const std::os::raw::c_void) {
    
    unsafe {
        if let Some(msg) = msgPtr.as_ref() {
            let section = read_cstring(msg.section);
            let row = msg.row as u32;
            let col = msg.col as u32;
            let message = read_cstring(msg.message);
            let prefix = match msg.type_ {
                0 => "[ERROR]",
                1 => "[WARNING]",
                _ => "[DEBUG]"
            };
            println!("{} ({}, {}) : {} : {}", section, row, col, prefix, message);
        }
    }
}

fn main() {

    let as_version: String = angelscript::get_library_version();
    println!("Angelscript Version: {}", as_version);

    let as_options: String = angelscript::get_library_options();
    println!("Angelscript Library Options: {}", as_options);

    let mut engine = angelscript::engine::ScriptEngine::new();
    
    let glob_func_count = engine.get_global_function_count();
    println!("Num registered global functions: {}", glob_func_count);

    //pub type asFUNCTION_t = ::std::option::Option< unsafe extern "C" fn() >;
    type cbFUNCTION_t = std::option::Option< unsafe extern "C" fn(*const asSMessageInfo, *const std::os::raw::c_void) >;
    let base_fn: cbFUNCTION_t = Some(callback);
    
    let cb = unsafe {std::mem::transmute::<cbFUNCTION_t, asFUNCTION_t>(base_fn) };
    engine.set_message_callback(&cb);


    engine.send_message("section", 0, 1, asEMsgType_asMSGTYPE_INFORMATION, "direct engine message");
    
    as_log_debug!(engine, "macro debug message!");
    as_log_warning!(engine, "macro warning message!");
    as_log_error!(engine, "macro error message!");
}