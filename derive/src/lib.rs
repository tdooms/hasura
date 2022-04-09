extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field, Ident, Lit, Meta, NestedMeta, Type};

struct StructInfo {
    ident: Ident,
    name: String,
    pk: String,
    fields: Vec<(Ident, Type)>,
}

impl ToTokens for StructInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name,
            ident,
            pk,
            fields,
        } = self;

        let pk_name = Ident::new(&format!("{}Pk", ident), ident.span());

        let (pk_ident, pk_ty) = fields.iter().find(|(ident, _)| ident == pk).unwrap();

        let idents = fields.iter().map(|(ident, _)| quote!(#ident: {}));

        let params = fields
            .iter()
            .map(|(ident, _)| quote!(api::Encode::encode(&self.#ident)));

        let field_elems = fields.iter().map(|(ident, _)| quote!(Self::#ident()));

        let field_fns = fields.iter().map(|(ident, _)| {
            quote!(pub fn #ident<'a>() -> api::Field<'a, Self> { api::Field::new(stringify!(#ident)) })
        });

        let stuff = quote! {
            struct #pk_name {
                #pk_ident: #pk_ty
            }

            impl std::fmt::Display for #pk_name {

                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "id: {}", api::Encode::encode(&self.#pk_ident))
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

    let meta_mapper = |nested| match nested {
        NestedMeta::Meta(Meta::NameValue(nv)) => Some((nv.path, nv.lit)),
        _ => None,
    };

    let attributes = match ast.attrs.first().unwrap().parse_meta().unwrap() {
        Meta::List(list) => list
            .nested
            .into_iter()
            .filter_map(meta_mapper)
            .collect::<Vec<_>>(),
        _ => unimplemented!(),
    };

    let name = attributes
        .iter()
        .find(|(path, _)| path.segments[0].ident == "name");
    let pk = attributes
        .iter()
        .find(|(path, _)| path.segments[0].ident == "pk");

    let name_val = match &name.unwrap().1 {
        Lit::Str(str) => str.value(),
        _ => unimplemented!(),
    };

    let pk_val = match &pk.unwrap().1 {
        Lit::Str(str) => str.value(),
        _ => unimplemented!(),
    };

    let to_field = |field: &Field| (field.ident.clone().unwrap(), field.ty.clone());
    let fields: Vec<_> = raw.iter().map(to_field).collect();

    let info = StructInfo {
        ident,
        name: name_val,
        pk: pk_val,
        fields,
    };

    info.into_token_stream().into()
}
