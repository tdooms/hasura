use crate::{Fields, Object};
use itertools::Itertools;
use std::fmt::Formatter;

pub fn construct_query<T: Object>(
    f: &mut Formatter<'_>,
    name: impl ToString,
    params: &[(Option<&str>, String)],
    returning: &Fields<T>,
    affected_rows: bool,
    explicit_returning: bool,
) -> std::fmt::Result {
    let fmt_param = |(k, v): &(Option<&str>, String)| match k {
        Some(k) => format!("{k}: {v}"),
        None => v.clone(),
    };

    let params = match params.is_empty() {
        true => String::new(),
        false => format!("({})", params.into_iter().map(fmt_param).join(", ")),
    };

    let returns = match affected_rows {
        true => format!("{returning} affected_rows"),
        false => format!("{returning}"),
    };

    let returns = match explicit_returning {
        true => format!("returning {{ {} }}", returns),
        false => returns,
    };

    write!(f, "{}{} {{ {} }}", name.to_string(), params, returns)
}
