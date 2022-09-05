use attributes::{Field, Fields};
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub trait Object {
    type Pk: serde::Serialize;
    fn name<'a>() -> &'a str;
    fn all<'a>() -> Fields<'a, Self>
    where
        Self: Sized;

    fn except<'a>(fields: &[Field<'a, Self>]) -> Fields<'a, Self>
    where
        Self: Sized;
}

pub trait Queryable<P: Object>: Display {
    type Out: DeserializeOwned;
    fn name() -> String;
}

pub trait Mutation<P: Object>: Display {
    type Out: DeserializeOwned;
    fn name() -> String;
}
