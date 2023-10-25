extern crate proc_macro;
use proc_macro::TokenStream;

use syn;
use quote::quote;


#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_my_macro(&ast)
}

fn impl_my_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generation = quote! {
        impl MyMacro for #name {
            fn my_macro(){
                println!("Hello, Macro, My name is {}", stringify!(#name));
            }
        }
    };
    generation.into()
}