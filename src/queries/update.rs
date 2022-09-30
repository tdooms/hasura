use crate::{Braced, Builder, Conditions, Fields, Hasura, Mutation, Serialized};
use serde::de::DeserializeOwned;
use std::fmt::{Display, Formatter};
use serde::Serialize;

pub struct Update<'a, T: Hasura> {
    pub set: &'a T,
    pub conditions: Conditions<'a, T>,
    pub affected_rows: bool,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura + Serialize> Update<'a, T> {
    pub fn new(set: &'a T) -> Self {
        Self {
            set,
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

impl<'a, T: Hasura + Serialize + DeserializeOwned> Mutation<T> for Update<'a, T> {
    type Out = Vec<T>;
    fn name() -> String {
        format!("update_{}", T::table())
    }
}

impl<'a, T: Hasura + Serialize + DeserializeOwned> Display for Update<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .param("_set", &Serialized(&self.set))
            .param("where", &Braced(&self.conditions))
            .explicit(true)
            .write(f)
    }
}
