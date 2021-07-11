// extern crate proc_macro;
// use proc_macro::{TokenStream};
// use syn::{parse_macro_input, DeriveInput};
// use quote::{quote};

// #[proc_macro_derive(CollectionParentRouter)]
// pub fn collection_parent_router(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;
//     let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
//
//     let expanded = quote! {
//     impl #impl_generics crate::tree::TreeNode<S> for #name #ty_generics #where_clause {
//       // ...
//     }
//   };
//
//
//     let expanded = quote! {
//         impl<S> crate::::TreeNode<S> for #name<S>
//         where
//             S: ?Sized + Send + Sync + 'static,
//         {
//             fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>> {
//                 match syzygy::parts::Parts::from(path) {
//                     syzygy::parts::Parts::Nil => Some(syzygy::tree::collection::CollectionViewRouter::collection(&self).prepare(state)),
//                     syzygy::parts::Parts::Cons(id, rest) => match syzygy::parts::Parts::from(rest) {
//                         Parts::Nil => Some(syzygy::tree::item::CollectionViewRouter::item(&self).prepare(id.into(), state)),
//                         _ => None,
//                     },
//                 }
//             }
//         }
//     };
//     expanded.into()
// }

// #[proc_macro_derive(CollectionRouter)]
// pub fn collection_router(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let expanded = quote! {
//
//     };
//     TokenStream::from(expanded)
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
