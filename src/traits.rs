use crate::{Field, Fields};
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub trait Hasura {
    type Pk: serde::Serialize;

    fn table<'a>() -> &'a str;
    fn all<'a>() -> Fields<'a, Self> where Self: Sized;
    fn except<'a>(fields: &[Field<'a, Self>]) -> Fields<'a, Self> where Self: Sized;
}

pub trait Queryable<P: Hasura>: Display {
    type Out: DeserializeOwned;
    fn name() -> String;
}

pub trait Mutation<P: Hasura>: Display {
    type Out: DeserializeOwned;
    fn name() -> String;
}
