pub struct Attributes {
    pub name: String,
    pub pks: Vec<String>,
    pub draft: Option<String>,
}

impl Attributes {
    pub fn from_syn_attrs(attrs: &[syn::Attribute]) -> Result<Self, String> {
        let attr = attrs
            .first()
            .ok_or_else(|| "must specify name in attributes".to_owned())?;

        let meta_mapper = |nested| match nested {
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => Some((nv.path, nv.lit)),
            _ => None,
        };

        let kv = match attr
            .parse_meta()
            .map_err(|_| "invalid attribute".to_owned())?
        {
            syn::Meta::List(list) => list
                .nested
                .into_iter()
                .filter_map(meta_mapper)
                .collect::<Vec<_>>(),
            _ => return Err("must be attribute list".to_owned()),
        };

        let mut name = None;
        let mut pks = Vec::new();
        let mut draft = None;

        for (path, value) in kv {
            let key = &path.segments.first().ok_or("invalid name")?.ident;

            if let (true, syn::Lit::Str(value)) = (key == "name", &value) {
                name = Some(value.value());
            } else if let (true, syn::Lit::Str(value)) = (key == "pk", &value) {
                pks.push(value.value());
            } else if let (true, syn::Lit::Str(value)) = (key == "draft", &value) {
                draft = Some(value.value());
            } else {
                return Err(format!("invalid attribute: {}", key));
            }
        }

        let name = name.ok_or_else(|| "no name found".to_owned())?;

        Ok(Self { name, pks, draft })
    }
}
