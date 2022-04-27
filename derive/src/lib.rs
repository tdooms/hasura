extern crate proc_macro;

use proc_macro::TokenStream;

use crate::attributes::Attributes;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

use crate::fields::Field;
use crate::generate::{EncodeInfo, ObjectInfo, PkInfo};

mod attributes;
mod fields;
mod generate;

#[proc_macro_derive(Object, attributes(name, object))]
pub fn object_derive(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    let fields = Field::from_ast_data(&ast.data).unwrap();
    let attrs = Attributes::from_syn_attrs(&ast.attrs).unwrap();

    let draft = attrs
        .draft
        .map(|name| syn::Ident::new(&name, ast.ident.span()))
        .unwrap_or_else(|| ast.ident.clone());

    let info = ObjectInfo {
        ident: ast.ident,
        name: attrs.name,
        draft,
        fields,
    };

    info.into_token_stream().into()
}

#[proc_macro_derive(Encode)]
pub fn encode_derive(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    let fields = Field::from_ast_data(&ast.data).unwrap();

    let info = EncodeInfo {
        ident: ast.ident,
        fields,
    };

    info.into_token_stream().into()
}

#[proc_macro_derive(Pk, attributes(pk))]
pub fn pk_derive(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    let fields = Field::from_ast_data(&ast.data).unwrap();
    let attrs = Attributes::from_syn_attrs(&ast.attrs).unwrap();

    let mut pks = vec![];
    for Field { ident, ty, .. } in fields.iter() {
        if attrs.pks.contains(&ident.to_string()) {
            pks.push((ident.clone(), ty.clone()))
        }
    }

    let info = PkInfo {
        ident: ast.ident,
        pks,
    };

    info.into_token_stream().into()
}
