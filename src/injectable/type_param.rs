use syn::parse::Parse;
use ::syn;

/// Type(Bound + Bound + Bound + ...) = Default
pub struct TypeParam(syn::TypeParam);

impl TypeParam {
    pub fn into_syn_generic_param(self) -> syn::GenericParam {
        let result: syn::TypeParam = self.into_syn_type_param();
        let result: syn::GenericParam = syn::GenericParam::Type(result);
        result
    }

    pub fn into_syn_type_param(self) -> syn::TypeParam {
        self.0
    }
}

pub fn parse(parse_stream: syn::parse::ParseStream) -> syn::Result<TypeParam> {
    TypeParam::parse(parse_stream)
}

impl syn::parse::Parse for TypeParam {
    fn parse(parse_stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = parse_stream.parse()?;
        let nested: syn::parse::ParseBuffer;
        syn::parenthesized!(nested in parse_stream);
        let _: Option<syn::Token![+]> = parse_stream.parse()?;
        let bounds: syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus> = nested.parse_terminated(::syn::TypeParamBound::parse, syn::Token![+])?;
        let default: Option<syn::Type> = if parse_stream.peek(syn::Token![=]) {
            let _: syn::Token![=] = parse_stream.parse()?;
            let t: syn::Type = parse_stream.parse()?;
            Some(t)
        } else {
            None
        };
        let result: syn::TypeParam = syn::TypeParam {
            attrs: vec![],
            ident: name,
            colon_token: if bounds.is_empty() { 
                None 
            } else { 
                Some(syn::token::Colon::default()) 
            },
            bounds,
            eq_token: if default.is_none() { 
                None 
            } else { 
                Some(syn::token::Eq::default()) 
            },
            default
        };
        let result: Self = Self(result);
        Ok(result)
    }
}
