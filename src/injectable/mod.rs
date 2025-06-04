pub mod attribute;
pub mod r#fn;
pub mod placeholder_constant;
pub mod placeholder_type;

pub fn dispatch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: syn::Item = syn::parse_macro_input!(input as syn::Item);
    match &mut input {
        syn::Item::Fn(item_fn) => {
            let item_fn: syn::ItemFn = item_fn.to_owned();
            let item_fn: syn::ItemFn = r#fn::consume(item_fn);
            let item_fn_expansion: proc_macro2::TokenStream = quote::quote! { #item_fn };
            item_fn_expansion.into()
        },
        syn::Item::Struct(item_struct) => {
            todo!()
        },
        _ => panic!()
    }
}