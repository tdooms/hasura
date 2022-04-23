use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

macro_rules! impl_encode {
    ($e:ty, $s:literal) => {
        impl Encode for $e {
            fn encode(&self) -> String {
                format!($s, self)
            }
        }
    };
}

pub trait Pk {
    type Pk: Display;
}

pub trait Object {
    fn name<'a>() -> &'a str;
}

pub trait Encode {
    fn encode(&self) -> String;
}

impl_encode!(String, "\\\"{}\\\"");

impl_encode!(u128, "\\\"{}\\\"");
impl_encode!(u64, "\\\"{}\\\"");
impl_encode!(u32, "\\\"{}\\\"");
impl_encode!(u16, "\\\"{}\\\"");
impl_encode!(u8, "\\\"{}\\\"");

impl_encode!(i128, "\\\"{}\\\"");
impl_encode!(i64, "\\\"{}\\\"");
impl_encode!(i32, "\\\"{}\\\"");
impl_encode!(i16, "\\\"{}\\\"");
impl_encode!(i8, "\\\"{}\\\"");

impl_encode!(f64, "\\\"{}\\\"");
impl_encode!(f32, "\\\"{}\\\"");

impl_encode!(isize, "\\\"{}\\\"");
impl_encode!(usize, "\\\"{}\\\"");

impl_encode!(bool, "{}");
impl_encode!(chrono::DateTime<chrono::Utc>, "\\\"{}\\\"");

impl<T: Encode> Encode for Option<T> {
    fn encode(&self) -> String {
        match self {
            Some(v) => v.encode(),
            None => "null".to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub inner: Vec<Field<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(name: &'a str) -> Self {
        Field {
            name,
            inner: vec![],
        }
    }
    pub fn recursive(name: &'a str, inner: Vec<Field<'a>>) -> Self {
        Field { name, inner }
    }
}

impl<'a> Display for Field<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner.is_empty() {
            true => f.write_str(self.name),
            false => write!(f, "{} {{ {} }}", self.name, self.inner.iter().join(", ")),
        }
    }
}

#[derive(Clone)]
pub struct Condition<'a> {
    pub op: &'a str,
    pub value: &'a str,
}

impl<'a> Display for Condition<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.op, self.value)
    }
}

#[derive(Clone)]
pub enum Conditions<'a> {
    And(Vec<Conditions<'a>>),
    Or(Vec<Conditions<'a>>),
    Not(Vec<Conditions<'a>>),
    Field(Field<'a>, Vec<Condition<'a>>),
}

impl<'a> Conditions<'a> {
    pub fn and(self, other: Self) -> Self {
        Self::And(vec![self, other])
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(vec![self, other])
    }

    pub fn not(self, other: Self) -> Self {
        Self::Not(vec![self, other])
    }
}

impl<'a> Display for Conditions<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And(v) => write!(f, "_and{{ {} }}", v.iter().format(", ")),
            Self::Or(v) => write!(f, "_or{{ {} }}", v.iter().format(", ")),
            Self::Not(v) => write!(f, "_not{{ {} }}", v.iter().format(", ")),
            Self::Field(field, cond) => write!(f, "{}: {{ {} }}", field, cond.iter().format(", ")),
        }
    }
}

#[derive(derive_more::Display, Clone)]
pub enum OrderBy<'a> {
    #[display(fmt = "{}: asc", _0)]
    Asc(Field<'a>),
    #[display(fmt = "{}: asc_nulls_first", _0)]
    AscNullsFirst(Field<'a>),
    #[display(fmt = "{}: asc_nulls_last", _0)]
    AscNullsLast(Field<'a>),
    #[display(fmt = "{}: desc", _0)]
    Desc(Field<'a>),
    #[display(fmt = "{}: desc_nulls_first", _0)]
    DescNullsFirst(Field<'a>),
    #[display(fmt = "{}: desc_nulls_last", _0)]
    DescNullsLast(Field<'a>),
}

pub struct OnConflict {
    // TODO: implement
}
