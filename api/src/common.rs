use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub trait Pk {
    type Pk: serde::Serialize;
}

pub trait Object {
    type Draft: serde::Serialize;
    fn name<'a>() -> &'a str;
    fn all<'a>() -> Fields<'a, Self>
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct Field<'a, T: Object + ?Sized> {
    pub name: &'a str,
    pub inner: Vec<String>,
    pub phantom: PhantomData<T>,
}

impl<'a, T: Object> Field<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Field {
            name,
            inner: vec![],
            phantom: PhantomData::default(),
        }
    }
    pub fn recursive<S: Object>(name: &'a str, keys: Fields<'a, S>) -> Self {
        let inner = keys.inner.into_iter().map(|k| k.to_string()).collect();
        Field {
            name,
            inner,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, T: Object> Display for Field<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner.is_empty() {
            true => f.write_str(self.name),
            false => write!(f, "{} {{ {} }}", self.name, self.inner.iter().join(" ")),
        }
    }
}

pub struct Fields<'a, T: Object + Sized> {
    pub inner: Vec<Field<'a, T>>,
}

impl<'a, T: Object> Display for Fields<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.iter().join(" "))
    }
}

impl<'a, T: Object> Default for Fields<'a, T> {
    fn default() -> Self {
        T::all()
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
pub enum Conditions<'a, T: Object> {
    And(Vec<Conditions<'a, T>>),
    Or(Vec<Conditions<'a, T>>),
    Not(Vec<Conditions<'a, T>>),
    Field(Field<'a, T>, Vec<Condition<'a>>),
}

impl<'a, T: Object> Conditions<'a, T> {
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

impl<'a, T: Object> Display for Conditions<'a, T> {
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

pub struct OnConflict {
    // TODO: implement
}
