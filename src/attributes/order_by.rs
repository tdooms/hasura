use std::fmt::{Display, Formatter};
use crate::{Field, Hasura};

#[derive(Clone)]
pub enum OrderBy<'a, T: Hasura> {
    Asc(Field<'a, T>),
    AscNullsFirst(Field<'a, T>),
    AscNullsLast(Field<'a, T>),
    Desc(Field<'a, T>),
    DescNullsFirst(Field<'a, T>),
    DescNullsLast(Field<'a, T>),
}

impl<'a, T: Hasura> Display for OrderBy<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Asc(field) => write!(f, "{field}: asc"),
            OrderBy::AscNullsFirst(field) => write!(f, "{field}: asc_nulls_first"),
            OrderBy::AscNullsLast(field) => write!(f, "{field}: asc_nulls_last"),
            OrderBy::Desc(field) => write!(f, "{field}: desc"),
            OrderBy::DescNullsFirst(field) => write!(f, "{field}: desc_nulls_first"),
            OrderBy::DescNullsLast(field) => write!(f, "{field}: desc_nulls_last"),
        }
    }
}
