mod impls;
mod trait_impl_fragment;

#[proc_macro]
pub fn impls(stream: ::proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(stream as header::Impls).into_token_stream().into()
}

mod header;