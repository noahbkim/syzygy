extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};
use quote::{quote};

#[proc_macro_derive(CollectionParentRouter)]
pub fn collection_parent_router(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {

    };
    expanded.into()
}

#[proc_macro_derive(CollectionRouter)]
pub fn collection_router(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {

    };
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
