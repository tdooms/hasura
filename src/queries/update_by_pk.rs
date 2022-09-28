use std::fmt::{Display, Formatter};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{Builder, Fields, Hasura, Mutation};
use crate::Serialized;

pub struct UpdateByPk<'a, T: Hasura> {
    pub pk: T::Pk,
    pub set: &'a T,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> UpdateByPk<'a, T> {
    pub fn new(pk: T::Pk, set: &'a T) -> Self {
        Self {
            pk,
            set,
            returning: T::all(),
        }
    }
    pub fn returning(mut self, returning: Fields<'a, T>) -> Self {
        self.returning = returning;
        self
    }
}

impl<'a, T: Hasura + Serialize + DeserializeOwned> Mutation<T> for UpdateByPk<'a, T> {
    type Out = Option<T>;
    fn name() -> String {
        format!("update_{}_by_pk", T::table())
    }
}

impl<'a, T: Hasura + Serialize + DeserializeOwned> Display for UpdateByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::new(Self::name(), &self.returning)
            .param("pk_columns", &Serialized(&self.pk))
            .param("_set", &Serialized(&self.set))
            .write(f)
    }
}