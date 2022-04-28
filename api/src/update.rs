use itertools::Itertools;

use crate::common::{Conditions, Pk};
use crate::util::construct_query;
use crate::{Encode, Field, Fields, Mutation, Object};
use std::fmt::Formatter;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Update<'a, T: Object + Encode> {
    pub set: T::Draft,
    pub conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    pub affected_rows: bool,
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + Encode> Mutation for Update<'a, T> {
    type Output = Vec<T>;
}

impl<'a, T: Object + Encode> std::fmt::Display for Update<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![(Some("_set"), format!("{{ {} }}", self.set.encode()))];

        if !self.conditions.is_empty() {
            let conditions = format!("{{ {} }}", self.conditions.iter().format(", "));
            params.push((Some("where"), conditions));
        }

        let name = format!("update_{}", T::name());

        let rows = self.affected_rows;
        construct_query(f, name, &params, &self.returning, rows)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct UpdateByPk<'a, T: Object + Encode + Pk> {
    pub pk: T::Pk,
    pub set: T::Draft,
    #[builder(default)]
    pub conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + Encode + Pk> Mutation for UpdateByPk<'a, T> {
    type Output = Option<T>;
}

impl<'a, T: Object + Encode + Pk> std::fmt::Display for UpdateByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = format!("update_{}_by_pk", T::name());

        let params = [
            (Some("_set"), self.set.encode()),
            (Some("pk_columns"), self.pk.to_string()),
        ];

        construct_query(f, name, &params, &self.returning, false)
    }
}
