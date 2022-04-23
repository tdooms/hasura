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
impl_encode!(u64, "\\\"{}\\\"");
impl_encode!(u32, "\\\"{}\\\"");
impl_encode!(i64, "\\\"{}\\\"");
impl_encode!(i32, "\\\"{}\\\"");
impl_encode!(isize, "\\\"{}\\\"");
impl_encode!(usize, "\\\"{}\\\"");
impl_encode!(bool, "{}");

impl<T: Encode> Encode for Option<T> {
    fn encode(&self) -> String {
        match self {
            Some(v) => v.encode(),
            None => "null".to_owned(),
        }
    }
}

impl Encode for chrono::DateTime<chrono::Utc> {
    fn encode(&self) -> String {
        format!("\\\"{}\\\"", self.to_rfc3339())
    }
}

#[derive(Clone)]
pub struct Field<'a, T: ?Sized> {
    pub name: &'a str,
    pub inner: Option<String>,
    pub phantom: PhantomData<T>,
}

impl<'a, T> Field<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Field {
            name,
            inner: None,
            phantom: Default::default(),
        }
    }
    pub fn recursive<S>(name: &'a str, inner: Vec<Field<'a, S>>) -> Self {
        Field {
            name,
            inner: Some(inner.into_iter().join(" ")),
            phantom: Default::default(),
        }
    }
}

impl<'a, T> Display for Field<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            None => f.write_str(self.name),
            Some(s) => write!(f, "{} {{ {} }}", self.name, s),
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
pub enum Conditions<'a, T> {
    And(Vec<Conditions<'a, T>>),
    Or(Vec<Conditions<'a, T>>),
    Not(Vec<Conditions<'a, T>>),
    Field(Field<'a, T>, Vec<Condition<'a>>),
}

impl<'a, T> Conditions<'a, T> {
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

impl<'a, T> Display for Conditions<'a, T> {
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
pub enum OrderBy<'a, T> {
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

pub struct OnConflict {
    // TODO: implement
}
