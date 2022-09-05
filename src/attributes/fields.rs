use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use itertools::Itertools;
use crate::traits::Hasura;

#[derive(Clone)]
pub struct Field<'a, T: Hasura + ?Sized> {
    pub name: &'a str,
    pub inner: Vec<String>,
    pub phantom: PhantomData<T>,
}

impl<'a, T: Hasura> PartialEq for Field<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl<'a, T: Hasura> Field<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Field {
            name,
            inner: vec![],
            phantom: PhantomData::default(),
        }
    }
    pub fn recursive<S: Hasura>(name: &'a str, keys: Fields<'a, S>) -> Self {
        let inner = keys.inner.into_iter().map(|k| k.to_string()).collect();
        Field {
            name,
            inner,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, T: Hasura> Display for Field<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner.is_empty() {
            true => write!(f, "{}", self.name),
            false => write!(f, "{} {{ {} }}", self.name, self.inner.iter().join(" ")),
        }
    }
}

#[derive(Clone)]
pub struct Fields<'a, T: Hasura + Sized> {
    pub inner: Vec<Field<'a, T>>,
}

impl<'a, T: Hasura> Display for Fields<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.iter().join(" "))
    }
}

impl<'a, T: Hasura> Default for Fields<'a, T> {
    fn default() -> Self {
        T::all()
    }
}
