pub mod angelscript;

use angelscript::engine::MessageInfo;

fn msg_callback(msg: MessageInfo) {
    let prefix = match msg.msg_type {
        0 => "[ERROR]",
        1 => "[WARNING]",
        _ => "[DEBUG]"
    };

    println!("{} ({}, {}) : {} : {}", msg.section, msg.row, msg.col, prefix, msg.message);
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

    engine.send_message("section", 0, 1, crate::angelscript::types::asEMsgType_asMSGTYPE_INFORMATION, "direct engine message");

    as_log_debug!(engine, "macro debug message!");
    as_log_warning!(engine, "macro warning message!");
    as_log_error!(engine, "macro error message!");
}