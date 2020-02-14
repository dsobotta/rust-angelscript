fn main()
{
    println!("cargo:rustc-link-lib=static=angelscript_c");
    println!("cargo:rustc-link-lib=static=angelscript");
    println!("cargo:rustc-link-lib=static=stdc++");

}
