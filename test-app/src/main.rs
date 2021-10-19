use angelscript::types::*;
use angelscript::engine::MessageInfo;
use angelscript::engine::ScriptEngine;
//use angelscript::module::ScriptModule;
use angelscript::context::ScriptContext;
use angelscript::as_log_debug;
use angelscript::as_log_warning;
use angelscript::as_log_error;
use angelscript::check_ok;

#[macro_use]
extern crate angelscript_derive;

fn msg_callback(msg: MessageInfo) {
    let prefix = match msg.msg_type {
        0 => "[ERROR]",
        1 => "[WARNING]",
        _ => "[DEBUG]"
    };

    println!("{} ({}, {}) : {} : {}", msg.section, msg.row, msg.col, prefix, msg.message);
}

fn test_core(engine: & ScriptEngine) {

    let as_version: String = angelscript::get_library_version();
    println!("Angelscript Version: {}", as_version);

    let as_options: String = angelscript::get_library_options();
    println!("Angelscript Library Options: {}", as_options);

    let glob_func_count = engine.get_global_function_count();
    println!("Num registered global functions: {}", glob_func_count);
}

fn test_callback(mut engine: &mut ScriptEngine) {

    let r = engine.set_message_callback(msg_callback);
    check_ok!(r);

    let r = engine.send_message("section", 0, 1, EMsgType::Warning, "direct engine message");
    check_ok!(r);

    as_log_debug!(engine, "macro debug message!");
    as_log_warning!(engine, "macro warning message!");
    as_log_error!(engine, "macro error message!");
}

fn test_script_main(engine: &mut ScriptEngine, ctx: &mut ScriptContext) {
    
    let mut module = engine.get_module("main", EGMFlags::AlwaysCreate).unwrap();

    let src = "int main() { return 57; }";
    let r = module.add_script_section("intmain", src);
    check_ok!(r);

    let r = module.build();
    check_ok!(r);

    let main_func = module.get_function_by_decl("int main()");
    let r = ctx.prepare(main_func);
    check_ok!(r);

    let r = ctx.execute();
    assert_eq!(r.to_u32(), EContextState::ExecutionFinished.to_u32());

    let val = ctx.get_return_dword();

    println!("int main() result = {}", val);
}

fn test_script_class(engine: &mut ScriptEngine, ctx: &mut ScriptContext) {
    
    let mut module = engine.get_module("class", EGMFlags::AlwaysCreate).unwrap();

    let src = "
        class Foo { 
            Foo() {} 
            int Bar() { return 53; } 
        }";
    
    let r = module.add_script_section("Foo", src);
    check_ok!(r);

    let r = module.build();
    check_ok!(r);

    let mut typeinfo = module.get_typeinfo_by_decl("Foo").unwrap();
    let factory = typeinfo.get_factory_by_decl("Foo @Foo()");

    let r = ctx.prepare(factory);
    check_ok!(r);

    let r = ctx.execute();
    assert_eq!(r.to_u32(), EContextState::ExecutionFinished.to_u32());

    let mut obj = ctx.get_return_object().unwrap();
    
    let func = typeinfo.get_method_by_decl("int Bar()");

    let r = ctx.prepare(func);
    check_ok!(r);

    let r = ctx.set_object(&mut obj);
    check_ok!(r);

    let r = ctx.execute();
    assert_eq!(r.to_u32(), EContextState::ExecutionFinished.to_u32());

    let val = ctx.get_return_dword();

    println!("Foo::Bar() result = {}", val);
}

#[as_function]
fn baz() { println!("baz()"); }

fn main() {

    let mut engine = ScriptEngine::new();
    let mut ctx = engine.create_context().unwrap();

    test_callback(&mut engine);

    test_core(&engine);

    test_script_main(&mut engine, &mut ctx);

    test_script_class(&mut engine, &mut ctx);

    println!("bindingstr = {}", angelscript_bindings::bindinfo_baz.as_decl);
    unsafe { angelscript_bindings::wrapped_baz(); }
}