pub mod angelscript;

use angelscript::read_cstring;
use angelscript::types::*;

unsafe extern "C" fn msg_callback(msg_ptr: *const asSMessageInfo, _params: *const std::os::raw::c_void) {
    if let Some(msg) = msg_ptr.as_ref() {
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

fn main() {

    let as_version: String = angelscript::get_library_version();
    println!("Angelscript Version: {}", as_version);

    let as_options: String = angelscript::get_library_options();
    println!("Angelscript Library Options: {}", as_options);

    let mut engine = angelscript::engine::ScriptEngine::new();
    
    let glob_func_count = engine.get_global_function_count();
    println!("Num registered global functions: {}", glob_func_count);


    engine.set_message_callback(msg_callback);

    engine.send_message("section", 0, 1, asEMsgType_asMSGTYPE_INFORMATION, "direct engine message");

    as_log_debug!(engine, "macro debug message!");
    as_log_warning!(engine, "macro warning message!");
    as_log_error!(engine, "macro error message!");
}