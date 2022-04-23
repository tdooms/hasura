#[derive(Clone, Debug)]
pub struct Field {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub expand: bool,
}

impl Field {
    pub fn from_ast_data(data: &syn::Data) -> Option<Vec<Self>> {
        let r#struct = match data {
            syn::Data::Struct(r#struct) => r#struct,
            _ => return None,
        };

        let named = match r#struct {
            syn::DataStruct {
                fields: syn::Fields::Named(named),
                ..
            } => named,
            _ => return None,
        };

        let to_field = |field: &syn::Field| {
            let ident = field.ident.clone().unwrap();
            let ty = field.ty.clone();

            let find_expand = |attr: &&syn::Attribute| attr.path.is_ident("object");
            let expand = field.attrs.iter().find(find_expand).is_some();

            Field { ident, ty, expand }
        };

        let fields: Vec<_> = named.named.iter().map(to_field).collect();
        Some(fields)
    }
}
