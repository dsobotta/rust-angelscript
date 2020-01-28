extern crate cmake;

use cmake::Config;
//use std::env;

fn main()
{
    let cfg = Config::new("cpp/angelscript/projects/cmake").build();
    Config::new("cpp").build();            

    println!("cargo:rustc-link-search=native={}", cfg.display());

    println!("cargo:rustc-link-lib=static=angelscript");
    println!("cargo:rustc-link-lib=static=angelscript_c");
}
