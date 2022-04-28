use std::fmt::Formatter;

use crate::{Field, Fields, Object};

pub fn construct_query<T: Object>(
    f: &mut Formatter<'_>,
    name: impl ToString,
    params: &[(Option<&str>, String)],
    returning: &Fields<T>,
    affected_rows: bool,
) -> std::fmt::Result {
    let fmt_param = |(k, v): &(Option<&str>, String)| match k {
        Some(k) => format!("{k}: {v}"),
        None => v.clone(),
    };

    let params: Vec<_> = params.into_iter().map(fmt_param).collect();

    let returns = match affected_rows {
        true => format!("{returning} affected_rows"),
        false => format!("{returning}"),
    };

    write!(
        f,
        "{}({}) {{ {} }}",
        name.to_string(),
        params.join(", "),
        returns
    )
}
