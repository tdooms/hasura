use std::fmt::{Display, Formatter};
use serde::de::DeserializeOwned;
use crate::{Builder, Fields, Hasura, Mutation, Serialized};

pub struct DeleteByPk<'a, T: Hasura> {
    pub pk: T::Pk,
    pub affected_rows: bool,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> DeleteByPk<'a, T> {
    pub fn new(pk: T::Pk) -> Self {
        Self {
            pk,
            affected_rows: false,
            returning: T::all(),
        }
    }
    pub fn returning(mut self, returning: Fields<'a, T>) -> Self {
        self.returning = returning;
        self
    }
}

impl<'a, T: Hasura + DeserializeOwned> Mutation<T> for DeleteByPk<'a, T> {
    type Out = Vec<T>;
    fn name() -> String {
        format!("delete_{}_by_pk", T::table())
    }
}

impl<'a, T: Hasura + DeserializeOwned> Display for DeleteByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .pk(&Serialized(&self.pk))
            .write(f)
    }
}