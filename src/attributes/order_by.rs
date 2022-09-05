#[derive(derive_more::Display, Clone)]
pub enum OrderBy<'a, T: Object> {
    #[display(fmt = "{}: asc", _0)]
    Asc(Field<'a, T>),
    #[display(fmt = "{}: asc_nulls_first", _0)]
    AscNullsFirst(Field<'a, T>),
    #[display(fmt = "{}: asc_nulls_last", _0)]
    AscNullsLast(Field<'a, T>),
    #[display(fmt = "{}: desc", _0)]
    Desc(Field<'a, T>),
    #[display(fmt = "{}: desc_nulls_first", _0)]
    DescNullsFirst(Field<'a, T>),
    #[display(fmt = "{}: desc_nulls_last", _0)]
    DescNullsLast(Field<'a, T>),
}
