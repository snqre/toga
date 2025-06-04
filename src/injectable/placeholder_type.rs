use ::syn::token;
use ::syn::parse;


/// Type(Bound + Bound + Bound + ...) = Default
#[derive(Clone)]
pub struct PlaceholderType {
    pub name: syn::Ident,
    pub constraints: Vec<syn::Path>,
    pub default: Option<syn::Ident>
}

impl PlaceholderType {
    pub fn into_syn_generic_param(self) -> syn::GenericParam {
        syn::GenericParam::Type(self.into_syn_type_param())
    }

    pub fn into_syn_type_param(self) -> syn::TypeParam {
        syn::TypeParam {
            attrs: vec![],
            ident: self.name.to_owned(),
            colon_token: if self.constraints.is_empty() {
                None
            } else {
                Some(token::Colon::default())
            },
            bounds: self.syn_type_param_bounds().into_iter().collect(),
            eq_token: self.default.as_ref().map(|_| token::Eq::default()),
            default: self.default.map(|ident| syn::Type::Path(syn::TypePath {
                qself: None,
                path: ident.into()
            }))
        }
    }

    pub fn syn_trait_bounds(&self) -> Vec<syn::TraitBound> {
        self.constraints
            .to_owned()
            .into_iter()
            .map(|path| syn::TraitBound {
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path
            })
            .collect()
    }

    pub fn syn_type_param_bounds(&self) -> Vec<syn::TypeParamBound> {
        self
            .syn_trait_bounds()
            .into_iter()
            .map(|syn_trait_bound| syn::TypeParamBound::Trait(syn_trait_bound))
            .collect()
    }

    fn parse_constraints(input: parse::ParseStream) -> syn::Result<Vec<syn::Path>> {
        if !input.peek(token::Paren) {
            return Ok(vec![])
        }
        let content: parse::ParseBuffer;
        syn::parenthesized!(content in input);
        let _: Option<token::Plus> = Self::parse_optional_leading_plus(&content);
        Self::parse_constraint_list(&content)
    }

    fn parse_constraint_list(content: parse::ParseStream) -> syn::Result<Vec<syn::Path>> {
        let mut constraints = Vec::new();
        if content.is_empty() {
            return Ok(constraints)
        }
        constraints.push(Self::parse_constraint(&content)?);
        while content.peek(syn::Token![+]) {
            content.parse::<syn::Token![+]>()?;
            if content.is_empty() {
                return Err(syn::Error::new(content.span(), "Trailing `+` without a following constraint"))
            }
            constraints.push(Self::parse_constraint(&content)?);
        }
        Ok(constraints)
    }

    fn parse_optional_leading_plus(content: parse::ParseStream) -> Option<syn::Token![+]> {
        content.parse::<syn::Token![+]>().ok()
    }

    fn parse_constraint(content: parse::ParseStream) -> syn::Result<syn::Path> {
        content.parse()
    }

    fn parse_default(input: parse::ParseStream) -> syn::Result<Option<syn::Ident>> {
        if input.peek(syn::Token![=]) {
            input.parse::<syn::Token![=]>()?;
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

impl parse::Parse for PlaceholderType {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {        
        let res: Self = Self {
            name: input.parse()?,
            constraints: Self::parse_constraints(input)?,
            default: Self::parse_default(input)?
        };
        Ok(res)
    }
}