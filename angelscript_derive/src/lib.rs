extern crate proc_macro;

use crate::proc_macro::*;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn as_function(_attr: TokenStream, input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as ItemFn);
    let fn_sig = &input.sig;
    let fn_ident = &fn_sig.ident;
    let fn_args = &fn_sig.inputs;

    let wrapper_ident = format_ident!("wrapped_{}", &fn_ident);
    let mut wrapper_sig = fn_sig.clone();
    wrapper_sig.ident = wrapper_ident;

    let bindinfo_ident = format_ident!("bindinfo_{}", &fn_ident);
    let as_decl = format!("void {}()", fn_ident.to_string() );

    let tokens = quote! {
        pub mod angelscript_bindings {
             pub unsafe extern "C" #wrapper_sig {
                 super::#fn_ident( #fn_args )
             }
            pub static #bindinfo_ident: angelscript::types::FuncBindInfo = angelscript::types::FuncBindInfo {
                func: None,
                as_decl: #as_decl,
            };
        }

        #input
    };

    tokens.into()
}