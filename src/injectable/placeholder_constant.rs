use ::syn::parse;

/// const LENGTH: usize
#[repr(transparent)]
pub struct PlaceholderConstant(syn::ConstParam);

impl parse::Parse for PlaceholderConstant {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}