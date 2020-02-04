pub mod angelscript;

//use angelscript::*;

fn main() {

    let as_version: String = angelscript::get_library_version();
    println!("Angelscript Version: {}", as_version);

    let as_options: String = angelscript::get_library_options();
    println!("Angelscript Library Options: {}", as_options);

    let engine = angelscript::engine::ScriptEngine::new();
    
    let glob_func_count = engine.get_global_function_count();
    println!("Num registered global functions: {}", glob_func_count);
}