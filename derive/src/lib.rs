extern crate proc_macro;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::{parse_macro_input, Attribute, DeriveInput};

use crate::fields::Field;
use crate::generate::{ObjectInfo, PkInfo};

mod fields;
mod generate;

#[proc_macro_derive(Object, attributes(name, object))]
pub fn object_derive(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    let fields = Field::from_ast_data(&ast.data).unwrap();

    let find_attr = |attr: &Attribute, name| attr.path.segments.first().unwrap().ident == name;

    let attr = ast.attrs.into_iter().find(|a| find_attr(a, "name"));
    let meta = attr.unwrap().parse_meta().unwrap();

    let nested = match meta {
        syn::Meta::List(syn::MetaList { nested, .. }) => nested,
        _ => panic!("attribute must be a list"),
    };
    let name = match nested.first() {
        Some(syn::NestedMeta::Lit(syn::Lit::Str(ref s))) => s.value(),
        _ => panic!("attribute must be a string"),
    };

    let info = ObjectInfo {
        ident: ast.ident,
        name,
        fields,
    };

    info.into_token_stream().into()
}

#[proc_macro_derive(Pk, attributes(pk))]
pub fn pk_derive(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    let fields = Field::from_ast_data(&ast.data).unwrap();

    let find_attr = |attr: &Attribute, name| attr.path.segments.first().unwrap().ident == name;

    let attr = ast.attrs.into_iter().find(|a| find_attr(a, "pk"));
    let meta = attr.unwrap().parse_meta().unwrap();

    let nested = match meta {
        syn::Meta::List(syn::MetaList { nested, .. }) => nested,
        _ => panic!("attribute must be a list"),
    };

    let map_pks = |n| match n {
        syn::NestedMeta::Lit(syn::Lit::Str(ref s)) => s.value(),
        _ => panic!("attribute must be a string"),
    };

    let keys = nested.into_iter().map(map_pks).collect::<Vec<_>>();

    let mut pks = vec![];
    for Field { ident, ty, .. } in fields.iter() {
        if keys.contains(&ident.to_string()) {
            pks.push((ident.clone(), ty.clone()))
        }
    }

    let info = PkInfo {
        ident: ast.ident,
        pks,
    };

    info.into_token_stream().into()
}
