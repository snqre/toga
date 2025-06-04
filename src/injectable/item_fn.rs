use super::*;

#[derive(Clone)]
pub struct ItemFn(syn::ItemFn);

pub fn parse(item_fn: syn::ItemFn) -> syn::Result<ItemFn> {
    let item_fn: ItemFn = ItemFn(item_fn);
    let result: &mut ItemFn = &mut item_fn.to_owned();
    let item_fn_attrs: Vec<attribute::Attribute> = item_fn.into_attrs();
    for attr in item_fn_attrs {
        match attr {
            attribute::Attribute::TypeParam(type_param) => {
                let generic_param = type_param.into_syn_generic_param();
                result.0.sig.generics.params.push(generic_param);
            },
            _ => todo!()
        }
    }
    let result: ItemFn = result.to_owned();
    Ok(result)
}

impl ItemFn {
    pub fn into_syn_item_fn(self) -> syn::ItemFn {
        self.0
    }
}

impl ItemFn {
    fn into_attrs(self) -> Vec<attribute::Attribute> {
        let mut attrs: Vec<attribute::Attribute> = vec![];
        let at_fn: Vec<attribute::Attribute> = self.attrs_at_fn_lvl();
        let at_fn_input: Vec<attribute::Attribute> = self.attrs_at_fn_input_lvl();
        attrs.extend(at_fn);
        attrs.extend(at_fn_input);
        self.strip();
        attrs
    }
}

impl ItemFn {
    fn attrs(&self) -> Vec<attribute::Attribute> {
        let mut attrs: Vec<attribute::Attribute> = vec![];
        let at_fn: Vec<attribute::Attribute> = self.attrs_at_fn_lvl();
        let at_fn_input: Vec<attribute::Attribute> = self.attrs_at_fn_input_lvl();
        attrs.extend(at_fn);
        attrs.extend(at_fn_input);
        attrs
    }

    fn attrs_at_fn_lvl(&self) -> Vec<attribute::Attribute> {
        let mut attrs: Vec<attribute::Attribute> = vec![];
        for attr in &self.0.attrs {
            if attr.path().is_ident("inject") {
                if let Ok(type_param) = attr.parse_args::<type_param::TypeParam>() {
                    let attr: attribute::Attribute = attribute::Attribute::TypeParam(type_param);
                    attrs.push(attr);
                }
            }
        }
        attrs
    }

    fn attrs_at_fn_input_lvl(&self) -> Vec<attribute::Attribute> {
        let mut attrs: Vec<attribute::Attribute> = vec![];
        for input in &self.0.sig.inputs {
            if let syn::FnArg::Typed(pat_ty) = input {
                for attr in &pat_ty.attrs {
                    if attr.path().is_ident("inject") {
                        if let Ok(type_param) = attr.parse_args::<type_param::TypeParam>() {
                            let attr: attribute::Attribute = attribute::Attribute::TypeParam(type_param);
                            attrs.push(attr);
                        }
                    }
                }
            }
        }
        attrs
    }
}

impl ItemFn {
    fn strip(&self) -> Self {
        self.strip_attrs_at_fn_lvl().strip_attrs_at_fn_input_lvl()
    }

    fn strip_attrs_at_fn_lvl(&self) -> Self {
        let mut item_fn = self.0.to_owned();
        let mut item_fn_attrs: Vec<syn::Attribute> = vec![];
        for attr in item_fn.attrs {
            if !attr.path().is_ident("inject") {
                item_fn_attrs.push(attr);
            }
        }
        item_fn.attrs = item_fn_attrs;
        Self(item_fn)
    }

    fn strip_attrs_at_fn_input_lvl(&self) -> Self {
        let mut item_fn: syn::ItemFn = self.0.to_owned();
        let mut item_fn_inputs = syn::punctuated::Punctuated::new();
        for input in item_fn.sig.inputs {
            match input {
                syn::FnArg::Typed(mut pat_ty) => {
                    let mut item_fn_input_attrs: Vec<syn::Attribute> = vec![];
                    for attr in pat_ty.attrs {
                        if !attr.path().is_ident("inject") {
                            item_fn_input_attrs.push(attr);
                        }
                    }
                    pat_ty.attrs = item_fn_input_attrs;
                    item_fn_inputs.push(syn::FnArg::Typed(pat_ty));
                },
                arg => item_fn_inputs.push(arg)
            }
        }
        item_fn.sig.inputs = item_fn_inputs;
        Self(item_fn)
    }
}