use crate::trait_impl_fragment;



pub struct Impls {
    pub header: Header,
    pub inherent_fn_items: Vec<InherentFnItem>,
    pub trait_impl_blocks: Vec<TraitImplBlockItem>
}

impl Impls {
    #[inline]
    pub fn into_token_stream(self) -> proc_macro2::TokenStream {
        self.header.into_token_stream(self.inherent_fn_items, self.trait_impl_blocks)
    }
}

impl syn::parse::Parse for Impls {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let header: Header = input.parse()?;
        let mut inherent_fn_items: Vec<_> = vec![];
        let mut trait_impl_blocks: Vec<_> = vec![];
        while !input.is_empty() {
            if input.peek(syn::Token![pub]) 
            || input.peek(syn::Token![fn])
            || input.peek(syn::Token![#]) {
                inherent_fn_items.push(
                    input.parse()?
                );
            } else {
                trait_impl_blocks.push(
                    input.parse()?
                );
            }
        }
        Ok(Self {
            header,
            inherent_fn_items,
            trait_impl_blocks
        })
    }
}




/// ```rs
/// impl<...> Type<...>
/// where
///     ...;
/// ```
pub struct Header {
    pub ty: syn::Type,
    pub ty_generics: Option<syn::Generics>,
    pub impl_generics: Option<syn::Generics>,
    pub where_clause: Option<syn::WhereClause>
}

impl Header {
    #[inline]
    pub fn into_token_stream(&self, inherent_fn_items: Vec<InherentFnItem>, trait_impl_blocks: Vec<TraitImplBlockItem>) -> proc_macro2::TokenStream {
        let header_ty: &syn::Type = &self.ty;
        let header_ty_generics: Option<_> = self.ty_generics.as_ref();
        let header_impl_generics: Option<_> = self.impl_generics.as_ref();
        let header_where_clause: Option<_> = self.where_clause.as_ref();
        let inherent_fn_item_expansions: Vec<_> = inherent_fn_items.iter().map(|item| {
            item.expansion()
        }).collect();
        let trait_impl_block_expansions: Vec<_> = trait_impl_blocks.iter().map(|item| {
            item.expansion(self)
        }).collect();
        if inherent_fn_item_expansions.is_empty() {
            return quote::quote!(
                #(#trait_impl_block_expansions)*
            )
        }
        quote::quote!(
            impl #header_impl_generics #header_ty #header_ty_generics
            #header_where_clause {
                #(#inherent_fn_item_expansions)*
            }

            #(#trait_impl_block_expansions)*
        )
    }
}

impl syn::parse::Parse for Header {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![impl]) {
            let _ = input.parse::<syn::Token![impl]>()?;
            let impl_generics: Option<syn::Generics> = input.parse().ok();
            let ty: syn::Type = input.parse()?;
            let ty_generics: Option<syn::Generics> = input.parse().ok();
            let where_clause: Option<syn::WhereClause> = input.parse().ok();
            let _ = input.parse::<syn::Token![;]>()?;
            return Ok(Self {
                ty,
                ty_generics,
                impl_generics,
                where_clause
            })
        }
        let ty: syn::Type = input.parse()?;
        let impl_generics: Option<syn::Generics> = input.parse().ok();
        let _: syn::Token![:] = input.parse()?;
        let ty_generics: Option<syn::Generics> = input.parse().ok();
        let where_clause: Option<syn::WhereClause> = input.parse().ok();
        let _ = input.parse::<syn::Token![;]>()?;
        Ok(Self {
            ty,
            ty_generics,
            impl_generics,
            where_clause
        })
    }
}








///```rs
/// Trait<...> {
///     ...
/// }
/// ```
pub struct TraitImplBlockItem {
    pub tr: syn::Path,
    pub tr_generics: Option<syn::Generics>,
    pub tr_block: syn::Block
}

impl TraitImplBlockItem {

    /// ```rs
    /// impl<...> Trait<...> for Type<...>
    /// where
    ///     ... {
    ///     ...
    /// }
    /// ```
    #[inline]
    pub fn expansion(&self, header: &Header) -> proc_macro2::TokenStream {
        let ty = &header.ty;
        let ty_generics = header.ty_generics.as_ref();
        let impl_generics = header.impl_generics.as_ref();
        let where_clause = header.where_clause.as_ref();
        let tr = &self.tr;
        let tr_generics = self.tr_generics.as_ref();
        let tr_block = &self.tr_block;
        quote::quote!(
            impl #impl_generics #ty_generics #tr #tr_generics for #ty #ty_generics
            #where_clause
            #tr_block
        )
    }
}

impl syn::parse::Parse for TraitImplBlockItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tr: syn::Path = input.parse()?;
        let tr_generics: Option<syn::Generics> = input.parse().ok();
        let tr_block: syn::Block = input.parse()?;
        Ok(Self {
            tr,
            tr_generics,
            tr_block
        })
    }
}


/// ```rs
/// #[...]
/// $vis $const fn $ident($inputs) -> $output
/// $where
///     ...
/// ```
pub struct InherentFnItem {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Option<syn::Visibility>,
    pub constant: Option<syn::Token![const]>,
    pub flag_async: Option<syn::Token![async]>,
    pub flag_unsafe: Option<syn::Token![unsafe]>,
    pub ident: syn::Ident,
    pub generics: Option<syn::Generics>,
    pub inputs: Vec<syn::FnArg>,
    pub output: Option<syn::ReturnType>,
    pub where_clause: Option<syn::WhereClause>,
    pub block: syn::Block
}

impl InherentFnItem {
    #[inline]
    fn expansion(&self) -> proc_macro2::TokenStream {
        let attrs = &self.attrs;
        let vis = self.vis.as_ref();
        let constant = self.constant.as_ref();
        let asy = self.flag_async.as_ref();
        let uns = self.flag_unsafe.as_ref();
        let ident = &self.ident;
        let generics = self.generics.as_ref();
        let inputs = &self.inputs;
        let output = self.output.as_ref();
        let where_clause = self.where_clause.as_ref();
        let block = &self.block;
        quote::quote!(
            #(#attrs)*
            #vis #constant #asy #uns fn #ident #generics (#(#inputs),*) #output
            #where_clause
            #block
        )
    }
}

impl syn::parse::Parse for InherentFnItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let vis: Option<syn::Visibility> = input.parse().ok();
        let constant: Option<_> = input.parse().ok();
        let flag_async: Option<_> = input.parse().ok();
        let flag_unsafe: Option<_> = input.parse().ok();
        let _ = input.parse::<syn::Token![fn]>()?;
        let ident: syn::Ident = input.parse()?;
        let generics: Option<syn::Generics> = input.parse().ok();
        let content: syn::parse::ParseBuffer<'_>;
        let _ = syn::parenthesized!(content in input);
        let inputs: Vec<syn::FnArg> = content.parse_terminated(syn::FnArg::parse, syn::Token![,])?.into_iter().collect();
        let output: Option<syn::ReturnType> = input.parse().ok();
        let where_clause: Option<syn::WhereClause> = input.parse().ok();
        let block: syn::Block = input.parse()?;
        Ok(Self {
            attrs,
            vis,
            constant,
            flag_async,
            flag_unsafe,
            ident,
            generics,
            inputs,
            output,
            where_clause,
            block
        })
    }
}