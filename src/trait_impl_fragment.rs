///
/// ```rs
/// Default {
///     fn default() -> Self {
///         Self
///     }
/// }
/// ```
pub struct TraitImplFragment {
    pub r#trait: syn::Path,
    pub r#trait_generics: Option<syn::Generics>,
    pub r#block: syn::Block
}

impl syn::parse::Parse for TraitImplFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let r#trait: syn::Path = input.parse()?;
        let r#trait_generics: Option<syn::Generics> = input.parse().ok();
        let r#block: syn::Block = input.parse()?;
        Ok(Self {
            r#trait,
            r#trait_generics,
            r#block
        })
    }
}