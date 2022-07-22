// !This is stolen from!
// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type_from_inner(ty: &syn::Type, inner:  Vec<&'static str>) -> Option<syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(typepath.path.clone()),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: Path, inner: Vec<&'static str>) -> Option<PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        inner
                .into_iter()
                .find(|s| &idents_of_path == *s)
                .and_then(move |_| path.segments.last().cloned())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path, inner))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first().cloned(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match generic_arg {
            GenericArgument::Type(ref ty) => Some(ty.clone()),
            _ => None,
        })
}

pub fn extract_type_from_option(ty: &syn::Type) -> Option<syn::Type> {
    let vec = vec!["Option|", "std|option|Option|", "core|option|Option|"];
    extract_type_from_inner(ty, vec)
}

pub fn extract_type_from_vec(ty: &syn::Type) -> Option<syn::Type> {
    let vec = vec!["Vec|", "std|vec|Vec|"];
    extract_type_from_inner(ty, vec)
}
