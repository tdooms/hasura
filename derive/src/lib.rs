use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[derive(FromField)]
#[darling(attributes(hasura))]
struct FieldOpts {
    ident: Option<syn::Ident>,
    // ty: syn::Type,

    #[darling(default)]
    relation: Option<String>,

    #[darling(default)]
    pk: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(hasura), supports(struct_any))]
struct TraitOpts {
    ident: syn::Ident,
    data: ast::Data<(), FieldOpts>,
    table: String,
}

impl ToTokens for TraitOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = match &self.data {
            ast::Data::Struct(fields) => &fields.fields,
            _ => unimplemented!(),
        };

        let pks: Vec<_> = fields
            .iter()
            .filter_map(|f| Some((&f.ident, syn::Ident::new(f.pk.as_ref()?, self.ident.span()))))
            .collect();


        let pk_fields = pks.iter().map(|(ident, pk)| quote!(pub #ident: #pk));
        let pk_params = pks.iter().map(|(ident, pk)| quote!(#ident: impl std::convert::Into<#pk>));
        let pk_intos = pks.iter().map(|(ident, _)| quote!(#ident: #ident.into()));

        let pk_ident = syn::Ident::new(&format!("{}Pk", self.ident), self.ident.span());

        let field_extractor = |field: &FieldOpts| {
            let ident = &field.ident;

            match field.relation {
                Some(_) => quote! {
                    pub fn #ident<'a, S: hasura::Hasura>(inner: hasura::Fields<'a, S>) -> hasura::Field<'a, Self> {
                        hasura::Field::recursive(stringify!(#ident), inner)
                    }
                },
                None => quote! {
                    pub fn #ident<'a>() -> hasura::Field<'a, Self> {
                        hasura::Field::new(stringify!(#ident))
                    }
                },
            }
        };

        let field_recursive = |field: &FieldOpts| {
            let ident = &field.ident;

            match &field.relation {
                Some(ty) => {
                    let ty = syn::Ident::new(&ty, self.ident.span());
                    quote! { Self::#ident(#ty::all()) }
                }
                None => quote! { Self::#ident() },
            }
        };

        let field_extractors = fields.iter().map(field_extractor);
        let field_recursives: Vec<_> = fields.iter().map(field_recursive).collect();

        let Self { ident, table, .. } = self;

        let stream = quote! {
            #[derive(serde::Serialize)]
            pub struct #pk_ident {
                #(#pk_fields),*
            }
            impl hasura::Hasura for #ident {
                type Pk = #pk_ident;

                fn table<'a>() -> &'a str {
                    #table
                }
                fn all<'a>() -> hasura::Fields<'a, Self> {
                    hasura::Fields{inner: vec![#(#field_recursives),*]}
                }
                fn except<'a>(fields: &[hasura::Field<'a, Self>]) -> hasura::Fields<'a, Self> {
                    let mut inner = vec![#(#field_recursives),*];
                    inner.retain(|f| !fields.contains(f));
                    hasura::Fields{inner}
                }
            }

            impl #ident {
                #(#field_extractors)*

                pub fn pk(#(#pk_params),*) -> #pk_ident {
                    #pk_ident {
                        #(#pk_intos),*
                    }
                }
            }
        };
        tokens.extend(stream);
    }
}

#[proc_macro_derive(Hasura, attributes(hasura))]
pub fn derive_hasura(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let opts = TraitOpts::from_derive_input(&parse_macro_input!(input)).unwrap();
    opts.to_token_stream().into()
}
