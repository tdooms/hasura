use itertools::Itertools;

use crate::common::Pk;
use crate::util::construct_query;
use crate::{serializer, Conditions, Fields, Mutation, Object};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Formatter;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Update<'a, T: Object + Serialize> {
    pub set: T::Draft,
    pub conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    pub affected_rows: bool,
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + DeserializeOwned + Serialize> Mutation<T> for Update<'a, T> {
    type Out = Vec<T>;

    fn name() -> String {
        format!("update_{}", T::name())
    }
}

impl<'a, T: Object + Serialize + DeserializeOwned> std::fmt::Display for Update<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![(
            Some("_set"),
            serializer::to_string(&self.set, true).unwrap(),
        )];

        if !self.conditions.is_empty() {
            let conditions = format!("{{ {} }}", self.conditions.iter().format(", "));
            params.push((Some("where"), conditions));
        }

        let name = Self::name();

        let rows = self.affected_rows;
        construct_query(f, &name, &params, &self.returning, rows, true)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct UpdateByPk<'a, T: Object + Serialize + Pk> {
    pub pk: T::Pk,
    pub set: T::Draft,
    #[builder(default)]
    pub conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + DeserializeOwned + Serialize + Pk> Mutation<T> for UpdateByPk<'a, T> {
    type Out = Option<T>;

    fn name() -> String {
        format!("update_{}_by_pk", T::name())
    }
}

impl<'a, T: Object + Serialize + Pk + DeserializeOwned> std::fmt::Display for UpdateByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = Self::name();

        let params = [
            (
                Some("_set"),
                serializer::to_string(&self.set, true).unwrap(),
            ),
            (
                Some("pk_columns"),
                serializer::to_string(&self.pk, false).unwrap(),
            ),
        ];

        construct_query(f, &name, &params, &self.returning, false, false)
    }
}
