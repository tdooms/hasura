use crate::Field;

#[derive(Debug, Clone, derive_more::Display)]
pub enum Kind {
    #[display(fmt = "query")]
    Query,
    #[display(fmt = "mutation")]
    Mutation,
}

pub fn construct_query<T>(
    kind: Kind,
    name: impl ToString,
    params: &[(Option<&str>, String)],
    returning: &[Field<T>],
    affected_rows: bool,
) -> String {
    let fmt_param = |(k, v): &(Option<&str>, String)| match k {
        Some(k) => format!("{k}: {v}"),
        None => v.clone(),
    };

    let params: Vec<_> = params.into_iter().map(fmt_param).collect();
    let mut returning: Vec<_> = returning.into_iter().map(|f| f.to_string()).collect();

    if affected_rows {
        returning.push("affected_rows".to_string());
    }

    format!(
        "{{ {}: {{ {}({}) {{ {} }} }} }}",
        kind,
        name.to_string(),
        params.join(", "),
        returning.join(" ")
    )
}
