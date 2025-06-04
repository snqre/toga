use super::*;

use ::syn;
use ::syn::parse;

/// #[inject(...)]
pub enum Attribute {
    PlaceholderConstant(placeholder_constant::PlaceholderConstant),
    TypeParam(type_param::TypeParam)
}

impl parse::Parse for Attribute {
    fn parse(parse_stream: parse::ParseStream) -> syn::Result<Self> {
        let nested: parse::ParseBuffer;
        syn::parenthesized!(nested in parse_stream);
        Ok(Attribute::TypeParam(type_param::parse(&nested)?))
    }
}