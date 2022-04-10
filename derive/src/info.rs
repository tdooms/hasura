use quote::{quote, ToTokens};
use syn::{Ident, Type};

pub struct StructInfo {
    pub ident: Ident,
    pub name: String,
    pub pks: Vec<(Ident, Type)>,
    pub fields: Vec<(Ident, Type)>,
}

impl ToTokens for StructInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name,
            ident,
            pks,
            fields,
        } = self;

        let pk_name = Ident::new(&format!("{}Pk", ident), ident.span());
        let pk_fields = pks.iter().map(|(ident, ty)| quote!(#ident: #ty));

        let idents = fields.iter().map(|(ident, _)| quote!(#ident: {}));
        let pk_idents = pks.iter().map(|(ident, _)| quote!(#ident: {}));

        let params = fields
            .iter()
            .map(|(ident, _)| quote!(api::Encode::encode(&self.#ident)));

        let pk_params = pks
            .iter()
            .map(|(ident, _)| quote!(api::Encode::encode(&self.#ident)));

        let field_elems = fields.iter().map(|(ident, _)| quote!(Self::#ident()));

        let field_fns = fields.iter().map(|(ident, _)| {
            quote!(pub fn #ident<'a>() -> api::Field<'a, Self> { api::Field::new(stringify!(#ident)) })
        });

        let stuff = quote! {
            struct #pk_name {
                #(#pk_fields),*
            }

            impl std::fmt::Display for #pk_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, stringify!(#(#pk_idents),*), #(#pk_params),*)
                }
            }

            impl #ident {
                pub fn all<'a>() -> Vec<api::Field<'a, Self>> { vec![#(#field_elems),*] }
                #(#field_fns)*
            }

            impl api::Object for #ident {
                type Pk = #pk_name;
                fn serialize(&self) -> String {
                    format!(stringify!(#(#idents),*), #(#params),*)
                }

                fn name<'a>() -> &'a str { #name }
            }
        };

        tokens.extend(stuff);
    }
}
