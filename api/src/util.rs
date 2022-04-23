use std::fmt::Formatter;

use crate::Field;

pub fn construct_query(
    f: &mut Formatter<'_>,
    name: impl ToString,
    params: &[(Option<&str>, String)],
    returning: &[Field],
    affected_rows: bool,
) -> std::fmt::Result {
    let fmt_param = |(k, v): &(Option<&str>, String)| match k {
        Some(k) => format!("{k}: {v}"),
        None => v.clone(),
    };

    let params: Vec<_> = params.into_iter().map(fmt_param).collect();
    let mut returning: Vec<_> = returning.into_iter().map(|f| f.to_string()).collect();

    if affected_rows {
        returning.push("affected_rows".to_string());
    }

    write!(
        f,
        "{}({}) {{ {} }}",
        name.to_string(),
        params.join(", "),
        returning.join(" ")
    )
}
