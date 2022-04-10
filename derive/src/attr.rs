pub struct AttrInfo {
    pub name: String,
    pub pks: Vec<String>,
}

impl AttrInfo {
    pub fn from_syn_attrs(attrs: &[syn::Attribute]) -> Result<AttrInfo, String> {
        let attr = attrs.first().ok_or("no attributes found".to_owned())?;

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

        for (path, value) in kv {
            let key = &path.segments.first().ok_or("invalid name")?.ident;

            if let (true, syn::Lit::Str(value)) = (key == "name", &value) {
                name = Some(value.value())
            } else if let (true, syn::Lit::Str(value)) = (key == "pk", &value) {
                pks.push(value.value())
            } else {
                return Err(format!("invalid attribute: {}", key));
            }
        }
        let name = name.ok_or("no name found".to_owned())?;
        Ok(AttrInfo { name, pks })
    }
}
