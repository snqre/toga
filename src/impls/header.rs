use crate::impls::trait_block;

pub struct Header {
    pub r#type: syn::Type,
    pub r#type_generics: Option<syn::Generics>,
    pub r#impl_generics: Option<syn::Generics>,
    pub r#where: Option<syn::WhereClause>
}

impl Header {

    pub fn resolve_inherent_block(&self, methods: &Vec<syn::ItemFn>) -> proc_macro2::TokenStream {
        let r#type: &syn::Type = &self.r#type;
        let r#type_generics: Option<&syn::Generics> = self.r#type_generics.as_ref();
        let r#impl_generics: Option<&syn::Generics> = self.r#impl_generics.as_ref();
        let r#where: Option<&syn::WhereClause> = self.r#where.as_ref();
        quote::quote! {
            impl #r#impl_generics #r#type #r#type_generics #r#where {
                #(#methods)*
            }
        }
    }

    pub fn resolve_trait_block(&self, trait_block: &trait_block::TraitBlock) -> proc_macro2::TokenStream {
        let r#type: &syn::Type = &self.r#type;
        let r#type_generics: Option<&syn::Generics> = self.r#type_generics.as_ref();
        let r#impl_generics: Option<&syn::Generics> = self.r#impl_generics.as_ref();
        let r#where: Option<&syn::WhereClause> = self.r#where.as_ref();
        let r#trait: &syn::Path = &trait_block.r#trait;
        let r#trait_generics: Option<&syn::Generics> = trait_block.r#trait_generics.as_ref();
        let block: &syn::Block = &trait_block.block;
        quote::quote! {
            impl #r#impl_generics #r#trait #r#trait_generics for #r#type #r#type_generics #r#where
            #block
        }
    }
}

impl syn::parse::Parse for Header {

    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = stream.parse::<syn::Token![impl]>()?;
        let r#impl_generics: Option<syn::Generics> = stream.parse().ok();
        let r#type: syn::Type = stream.parse()?;
        let r#type_generics: Option<syn::Generics> = stream.parse().ok();
        let r#where: Option<syn::WhereClause> = stream.parse().ok();
        let _ = stream.parse::<syn::Token![;]>()?;
        let header: Self = Self {
            r#type: r#type,
            r#type_generics: r#type_generics,
            r#impl_generics: r#impl_generics,
            r#where: r#where
        };
        Ok(header)
    }
}