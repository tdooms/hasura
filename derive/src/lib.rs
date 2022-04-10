mod attr;
mod info;

extern crate proc_macro;

use proc_macro::TokenStream;

use crate::attr::AttrInfo;
use crate::info::StructInfo;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Field};

#[proc_macro_derive(Object, attributes(object))]
pub fn object_derive(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);

    let raw = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let ident = ast.ident;

    let to_field = |field: &Field| (field.ident.clone().unwrap(), field.ty.clone());
    let fields: Vec<_> = raw.iter().map(to_field).collect();

    let attrs = AttrInfo::from_syn_attrs(&ast.attrs).unwrap();

    let mut pks = vec![];
    for (ident, ty) in fields.iter() {
        if let Some(_) = attrs.pks.iter().find(|&x| x == &ident.to_string()) {
            pks.push((ident.clone(), ty.clone()))
        }
    }

    let info = StructInfo {
        ident,
        name: attrs.name,
        pks,
        fields,
    };

    info.into_token_stream().into()
}
