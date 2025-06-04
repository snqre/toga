use super::*;

pub mod attribute;
pub mod item_fn;
pub mod placeholder_constant;
pub mod type_param;

pub fn dispatch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::Item = syn::parse_macro_input!(input as syn::Item);
    match input {
        syn::Item::Fn(item_fn) => {
            let item_fn: syn::ItemFn = item_fn::parse(item_fn).unwrap().into_syn_item_fn();
            let item_fn_expansion: proc_macro2::TokenStream = quote::quote! { #item_fn };
            item_fn_expansion.into()
        },
        syn::Item::Struct(_) => {
            todo!()
        },
        _ => panic!()
    }
}