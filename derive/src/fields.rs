pub struct Fields {
    pub fields: Vec<(syn::Ident, syn::Type)>,
}

impl Fields {
    pub fn from_ast_data(data: &syn::Data) -> Option<Self> {
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

        let to_field = |field: &syn::Field| (field.ident.clone().unwrap(), field.ty.clone());
        let fields: Vec<_> = named.named.iter().map(to_field).collect();
        Some(Self { fields })
    }
}
