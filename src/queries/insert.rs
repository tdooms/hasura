use crate::Separalized;
use crate::{Builder, Fields, Hasura, Mutation, OnConflict};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Formatter;

pub struct Insert<'a, T: Hasura> {
    pub objects: &'a [T],
    pub affected_rows: bool,
    pub on_conflict: Option<OnConflict>,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> Insert<'a, T> {
    pub fn new(objects: &'a [T]) -> Self {
        Insert {
            objects,
            affected_rows: false,
            on_conflict: None,
            returning: T::all(),
        }
    }
    pub fn affected_rows(mut self, affected_rows: bool) -> Self {
        self.affected_rows = affected_rows;
        self
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

impl<'a, T: Hasura + DeserializeOwned + Serialize> Mutation<T> for Insert<'a, T> {
    type Out = Vec<T>;
    fn name() -> String {
        format!("insert_{}", T::table())
    }
}

impl<'a, T: Hasura + DeserializeOwned + Serialize> std::fmt::Display for Insert<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .maybe("objects", &Separalized(self.objects.as_ref()))
            .maybe("on_conflict", &self.on_conflict)
            .affected(self.affected_rows)
            .explicit(true)
            .write(f)
    }
}
