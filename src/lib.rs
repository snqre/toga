#![allow(clippy::redundant_field_names)]
#![allow(clippy::let_with_type_underscore)]

mod blockset;

#[proc_macro]
pub fn blockset(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(stream as blockset::Blockset).resolve().into()
}