mod impls;
mod injectable;

#[proc_macro]
pub fn impls(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(stream as impls::Impls).resolve().into()
}