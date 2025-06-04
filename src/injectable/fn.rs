use super::*;

pub fn consume(item_fn: syn::ItemFn) -> syn::ItemFn {
    let mut item_fn: syn::ItemFn = item_fn;
    let mut attrs: Vec<attribute::Attribute> = vec![];
    attrs.extend(consume_attr_fn(&mut item_fn));
    attrs.extend(consume_attr_param(&mut item_fn));
    let mut r#gen = std::mem::take(&mut item_fn.sig.generics);
    for attr in attrs {
        match attr {
            attribute::Attribute::PlaceholderConstant(_) => {
                todo!()
            },
            attribute::Attribute::PlaceholderType(placeholder_type) => {
                let type_param: syn::GenericParam = placeholder_type.into_syn_generic_param();
                r#gen.params.push(type_param);
            }
        }
    }
    item_fn.sig.generics = r#gen;
    item_fn
}

fn consume_attr_param(item_fn: &mut syn::ItemFn) -> Vec<attribute::Attribute> {
    let mut res: Vec<attribute::Attribute> = vec![];
    for input in &mut item_fn.sig.inputs {
        if let syn::FnArg::Typed(pat_ty) = input {
            let mut keep_attrs: Vec<syn::Attribute> = vec![];
            for attr in &pat_ty.attrs {
                if attr.path().is_ident("inject") {
                    if let Ok(attr) = attr.parse_args::<attribute::Attribute>() { res.push(attr); }
                } else {
                    let attr: syn::Attribute = attr.to_owned();
                    keep_attrs.push(attr)
                }
            }
            pat_ty.attrs = keep_attrs;
        }
    }
    res
}

fn consume_attr_fn(item_fn: &mut syn::ItemFn) -> Vec<attribute::Attribute> {
    let mut res: Vec<attribute::Attribute> = vec![];
    let mut keep_attrs: Vec<syn::Attribute> = vec![];
    for attr in &item_fn.attrs {
        if attr.path().is_ident("inject") {
            if let Ok(attr) = attr.parse_args::<attribute::Attribute>() { res.push(attr); }
        } else {
            let attr: syn::Attribute = attr.to_owned();
            keep_attrs.push(attr);
        }
    }
    item_fn.attrs = keep_attrs;
    res
}