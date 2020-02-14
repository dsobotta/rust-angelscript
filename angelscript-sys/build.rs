extern crate cmake;

use cmake::Config;

fn main()
{
    let as_cfg = Config::new("cpp/angelscript/projects/cmake")
        .build();

    let asc_cfg = Config::new("cpp/add_on/clib")
        .build();            

    println!("cargo:rustc-link-search=native={}", as_cfg.display());
    println!("cargo:rustc-link-search=native={}", asc_cfg.display());

    //nightly variants to link but not bundle
    // println!("cargo:rustc-link-lib=static_nobundle=angelscript_c");
    // println!("cargo:rustc-link-lib=static_nobundle=angelscript");
    // println!("cargo:rustc-link-lib=static_nobundle=stdc++");

    //link as if it were an app
    // println!("cargo:rustc-link-lib=static=angelscript_c");
    // println!("cargo:rustc-link-lib=static=angelscript");
    // println!("cargo:rustc-link-lib=static=stdc++");

}
