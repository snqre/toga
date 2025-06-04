use super::*;

use ::syn::parse;

/// #[inject(...)]
pub enum Attribute {
    PlaceholderConstant(placeholder_constant::PlaceholderConstant),
    PlaceholderType(placeholder_type::PlaceholderType)
}

impl parse::Parse for Attribute {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let content: parse::ParseBuffer;
        syn::parenthesized!(content in input);
        if content.peek(syn::Token![const]) {
            return Ok(Attribute::PlaceholderConstant(placeholder_constant::PlaceholderConstant::parse(&content)?))
        }
        Ok(Attribute::PlaceholderType(placeholder_type::PlaceholderType::parse(&content)?))
    }
}