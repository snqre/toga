pub struct TraitBlock {
    pub r#trait: syn::Path,
    pub r#trait_generics: Option<syn::Generics>,
    pub block: syn::Block
}

impl syn::parse::Parse for TraitBlock {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let r#trait: syn::Path = stream.parse()?;
        let r#trait_generics: Option<syn::Generics> = stream.parse().ok();
        let block: syn::Block = stream.parse()?;
        let block: Self = Self { 
            r#trait, 
            r#trait_generics, 
            block
        };
        Ok(block)
    }
}