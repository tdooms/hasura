use std::fmt::Formatter;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{Builder, Fields, Hasura, Mutation, OnConflict};
use crate::Serialized;

pub struct InsertOne<'a, T: Hasura> {
    pub object: T,
    pub on_conflict: Option<OnConflict>,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> InsertOne<'a, T> {
    pub fn new(object: T) -> Self {
        InsertOne {
            object,
            on_conflict: None,
            returning: T::all(),
        }
    }
    pub fn on_conflict(mut self, on_conflict: OnConflict) -> Self {
        self.on_conflict = Some(on_conflict);
        self
    }
    pub fn returning(mut self, returning: Fields<'a, T>) -> Self {
        self.returning = returning;
        self
    }
}

impl<'a, T: Hasura + DeserializeOwned + Serialize> Mutation<T> for InsertOne<'a, T> {
    type Out = Option<T>;
    fn name() -> String { format!("insert_{}_one", T::table()) }
}

impl<'a, T: Hasura + DeserializeOwned + Serialize> std::fmt::Display for InsertOne<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .param("object", &Serialized(&self.object))
            .maybe("on_conflict", &self.on_conflict)
            .write(f)
    }
}