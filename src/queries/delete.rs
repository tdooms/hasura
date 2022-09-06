use crate::{Braced, Builder, Conditions, Fields, Hasura, Mutation};
use serde::de::DeserializeOwned;
use std::fmt::{Display, Formatter};

pub struct Delete<'a, T: Hasura> {
    pub conditions: Conditions<'a, T>,
    pub affected_rows: bool,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> Delete<'a, T> {
    pub fn new() -> Self {
        Self {
            conditions: Conditions::None,
            affected_rows: false,
            returning: T::all(),
        }
    }
    pub fn conditions(mut self, conditions: Conditions<'a, T>) -> Self {
        self.conditions = conditions;
        self
    }
    pub fn affected_rows(mut self, affected_rows: bool) -> Self {
        self.affected_rows = affected_rows;
        self
    }
    pub fn returning(mut self, returning: Fields<'a, T>) -> Self {
        self.returning = returning;
        self
    }
}

impl<'a, T: Hasura + DeserializeOwned> Mutation<T> for Delete<'a, T> {
    type Out = Vec<T>;
    fn name() -> String {
        format!("delete_{}", T::table())
    }
}

impl<'a, T: Hasura + DeserializeOwned> Display for Delete<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .param("where", &Braced(&self.conditions))
            .explicit(true)
            .write(f)
    }
}
