use angelscript::types::*;
use angelscript::engine::MessageInfo;
use angelscript::as_log_debug;
use angelscript::as_log_warning;
use angelscript::as_log_error;
use angelscript::check_ok;

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


    let r = engine.set_message_callback(msg_callback);
    check_ok!(r);

    let r = engine.send_message("section", 0, 1, EMsgType::Warning, "direct engine message");
    check_ok!(r);

    as_log_debug!(engine, "macro debug message!");
    as_log_warning!(engine, "macro warning message!");
    as_log_error!(engine, "macro error message!");

    let module = engine.get_module("test-module", EGMFlags::AlwaysCreate);
    match module {
        None => panic!("failed to get module"),
        Some(mut m) => {

            as_log_debug!(engine, "successfully loaded test-module");

            let int_main_src = "int main() { return 0; }";
            let r = m.add_script_section("intmain", int_main_src);
            check_ok!(r);
    
            let r = m.build();
            check_ok!(r);
        }
    }

    let context = engine.create_context();
    match context {
        None => panic!("failed to create context"),
        Some(mut ctx) => {
            as_log_debug!(engine, "successfully created context");
        }
    }
}