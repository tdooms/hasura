use std::fmt::{Display, Formatter};
use serde::de::DeserializeOwned;
use crate::{Builder, Fields, Hasura, Queryable, Serialized};

pub struct QueryByPk<'a, T: Hasura> {
    pub pk: T::Pk,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> QueryByPk<'a, T> {
    pub fn new(pk: T::Pk) -> Self {
        QueryByPk {
            pk,
            returning: T::all(),
        }
    }
    pub fn returning(mut self, returning: Fields<'a, T>) -> Self {
        self.returning = returning;
        self
    }
}

impl<'a, T: Hasura + DeserializeOwned> Queryable<T> for QueryByPk<'a, T> {
    type Out = Vec<T>;
    fn name() -> String {
        format!("{}_by_pk", T::table())
    }
}

impl<'a, T: Hasura + DeserializeOwned> Display for QueryByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .pk(&Serialized(&self.pk))
            .write(f)
    }
}