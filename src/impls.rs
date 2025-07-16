mod header;
mod trait_block;

pub struct Impls {
    pub header: header::Header,
    pub blocks: Vec<trait_block::TraitBlock>,
    pub methods: Vec<syn::ItemFn>
}

impl Impls {
    pub fn resolve(&self) -> proc_macro2::TokenStream {
        let blocks: _ = self.blocks.iter().map(|block| {
            let block: proc_macro2::TokenStream = self.header.resolve_trait_block(block);
            quote::quote! {
                #block
            }
        });
        let inherent = self.header.resolve_inherent_block(&self.methods);
        quote::quote! {
            #inherent
            #(#blocks)*
        }
    }
}

impl syn::parse::Parse for Impls {

    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let header: header::Header = stream.parse()?;
        let mut methods: Vec<syn::ItemFn> = vec![];
        let mut blocks: Vec<trait_block::TraitBlock> = vec![];
        while !stream.is_empty() {
            if stream.peek(syn::Token![fn]) | stream.peek(syn::Token![pub]) {
                let method: syn::ItemFn = stream.parse()?;
                methods.push(method)
            } else {
                let block: trait_block::TraitBlock = stream.parse()?;
                blocks.push(block)
            }
        }
        let blockset: Impls = Impls {
            header: header,
            blocks: blocks,
            methods: methods
        };
        Ok(blockset)
    }
}