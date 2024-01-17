use proc_macro::TokenStream;
use quote::format_ident;
use syn::DeriveInput;

#[proc_macro_derive(MakeStruct)]
pub fn struct_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();
    
    let ident = format_ident!("{}Item", ast.ident);
    let token_stream = quote::quote! {
        pub struct #ident {
            pub name: String,

            pub number: i8,
        }
    };
    
    token_stream.into()
}

