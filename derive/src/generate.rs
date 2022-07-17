use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::fields::Field;

pub struct PkInfo {
    pub ident: syn::Ident,
    pub pks: Vec<(syn::Ident, syn::Type)>,
}

impl ToTokens for PkInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, pks } = self;

        let pk_fields = pks.iter().map(|(ident, ty)| quote!(pub #ident: #ty));
        let pk_name = syn::Ident::new(&format!("{}Pk", ident), ident.span());

        let impls = quote! {
            #[derive(serde::Serialize)]
            pub struct #pk_name {
                #(#pk_fields,)*
            }

            impl hasura::Pk for #ident {
                type Pk = #pk_name;
            }
        };

        tokens.extend(impls);
    }
}

pub struct ObjectInfo {
    pub ident: syn::Ident,
    pub name: String,
    pub draft: syn::Ident,
    pub fields: Vec<Field>,
}

impl ToTokens for ObjectInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name,
            ident,
            fields,
            draft,
        } = self;

        let map_field_elems = |Field { ident, ty, expand }| match expand {
            false => quote!(Self::#ident()),
            true => quote!(Self::#ident(#ty::all())),
        };

        let field_elems = fields.iter().cloned().map(map_field_elems);

        let map_field_fn = |Field { ident, expand, .. }| {
            if expand {
                quote! {
                    pub fn #ident<'a, S: hasura::Object>(inner: hasura::Fields<'a, S>)
                        -> hasura::Field<'a, Self> {
                        hasura::Field::recursive(stringify!(#ident), inner)
                    }
                }
            } else {
                quote! {
                    pub fn #ident<'a>() -> hasura::Field<'a, Self> {
                        hasura::Field::new(stringify!(#ident))
                    }
                }
            }
        };

        let field_fns = fields.iter().cloned().map(map_field_fn);

        let impls = quote! {
            impl #ident {
                #(#field_fns)*
            }

            impl hasura::Object for #ident {
                type Draft = #draft;
                fn name<'a>() -> &'a str { #name }
                fn all<'a>() -> hasura::Fields<'a, Self> { hasura::Fields{inner: vec![#(#field_elems),*]} }
            }
        };

        tokens.extend(impls);
    }
}
