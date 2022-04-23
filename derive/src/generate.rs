use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::fields::Field;

// fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
//     if let syn::Type::Path(ref p) = ty {
//         if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
//             return None;
//         }
//
//         if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
//             if inner_ty.args.len() != 1 {
//                 return None;
//             }
//
//             if let syn::GenericArgument::Type(ref t) = inner_ty.args.first().unwrap() {
//                 return Some(t);
//             }
//         }
//     }
//     None
// }

fn encode_map(ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote!(hasura::Encode::encode(&self.#ident))
}

// TODO: incremental updates are not yet supported
// fn format_map(ident: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
//     match ty_inner_type("Option", ty) {
//         Some(ty) => quote!(#ident: #ty),
//         None => quote!(#ident: #ty),
//     }
// }

pub struct PkInfo {
    pub ident: syn::Ident,
    pub pks: Vec<(syn::Ident, syn::Type)>,
}

impl ToTokens for PkInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, pks } = self;

        let pk_fields = pks.iter().map(|(ident, ty)| quote!(pub #ident: #ty));
        let pk_idents = pks.iter().map(|(ident, _)| quote!(#ident: {}));

        let pk_params = pks.iter().map(|(ident, _)| encode_map(ident));
        let pk_name = syn::Ident::new(&format!("{}Pk", ident), ident.span());

        let impls = quote! {
            pub struct #pk_name {
                #(#pk_fields,)*
            }

            impl std::fmt::Display for #pk_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, stringify!(#(#pk_idents),*), #(#pk_params),*)
                }
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
    pub fields: Vec<Field>,
}

impl ToTokens for ObjectInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name,
            ident,
            fields,
        } = self;

        let idents = fields.iter().map(|Field { ident, .. }| quote!(#ident: {}));
        let params = fields.iter().map(|Field { ident, .. }| encode_map(ident));

        let map_field_elems = |Field { ident, ty, expand }| match expand {
            false => quote!(Self::#ident()),
            true => quote!(Self::#ident(#ty::all())),
        };

        let field_elems = fields.iter().cloned().map(map_field_elems);

        let map_field_fn = |Field { ident, ty, expand }| match expand {
            false => {
                quote! {
                    pub fn #ident<'a>() -> hasura::Field<'a> {
                        hasura::Field::new(stringify!(#ident))
                    }
                }
            }
            true => {
                quote! {
                    pub fn #ident<'a>(inner: std::vec::Vec<hasura::Field<'a>>)
                        -> hasura::Field<'a> {
                        hasura::Field::recursive(stringify!(#ident), inner)
                    }
                }
            }
        };

        let field_fns = fields.iter().cloned().map(map_field_fn);

        let impls = quote! {
            impl #ident {
                pub fn all<'a>() -> Vec<hasura::Field<'a>> { vec![#(#field_elems),*] }
                #(#field_fns)*
            }

            impl hasura::Encode for #ident {
                fn encode(&self) -> String{
                    format!(stringify!({} #(#idents),* {}), "{", #(#params),*, "}")
                }
            }

            impl hasura::Object for #ident {
                fn name<'a>() -> &'a str { #name }
            }
        };

        tokens.extend(impls);
    }
}
