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

    fn except<'a>(fields: &[Field<'a, Self>]) -> Fields<'a, Self>
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct Field<'a, T: Object + ?Sized> {
    pub name: &'a str,
    pub inner: Vec<String>,
    pub phantom: PhantomData<T>,
}

impl<'a, T: Object> PartialEq for Field<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
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
