use proc_macro::TokenStream;
use syn::spanned::Spanned;  // Add this import for the span() method
use quote::ToTokens;

mod impls;
mod injectable;

#[proc_macro]
pub fn impls(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(stream as impls::Impls).resolve().into()
}

#[proc_macro_attribute]
pub fn injectable(_: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    injectable::dispatch(input)
}

#[allow(non_snake_case)]
#[doc(hidden)]
#[proc_macro_attribute]
pub fn inject(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}