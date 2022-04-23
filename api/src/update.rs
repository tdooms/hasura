use itertools::Itertools;

use crate::common::{Conditions, Pk};
use crate::util::construct_query;
use crate::{Field, Mutation, Object};
use std::fmt::Formatter;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Update<'a, T: Object> {
    pub set: T,
    pub conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    pub affected_rows: bool,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> Mutation for Update<'a, T> {}

impl<'a, T: Object> std::fmt::Display for Update<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![(Some("_set"), format!("{{ {} }}", self.set.serialize()))];

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
pub struct UpdateByPk<'a, T: Object + Pk> {
    pub pk: T::Pk,
    pub set: T,
    #[builder(default)]
    pub conditions: Vec<Conditions<'a, T>>,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object + Pk> Mutation for UpdateByPk<'a, T> {}

impl<'a, T: Object + Pk> std::fmt::Display for UpdateByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pk = format!("{{ {} }}", self.pk.to_string());
        let name = format!("update_{}_by_pk", T::name());

        let params = [
            (Some("_set"), format!("{{ {} }}", self.set.serialize())),
            (Some("pk_columns"), pk),
        ];

        construct_query(f, name, &params, &self.returning, false)
    }
}
