extern crate cmake;

use cmake::Config;
//use std::env;

fn main()
{
    let as_cfg = Config::new("cpp/angelscript/projects/cmake")
        .build();

    let asc_cfg = Config::new("cpp")
        .build();            

    println!("cargo:rustc-link-search=native={}", as_cfg.display());
    println!("cargo:rustc-link-search=native={}", asc_cfg.display());

    println!("cargo:rustc-link-lib=static=angelscript_c");
    println!("cargo:rustc-link-lib=static=angelscript");
    println!("cargo:rustc-link-lib=static=stdc++");

}
