use crate::common::Pk;
use crate::util::construct_query;
use crate::{Conditions, Field, Mutation, Object};
use itertools::Itertools;
use std::fmt::Formatter;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Delete<'a, T: Object> {
    conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    affected_rows: bool,
    returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> Mutation for Delete<'a, T> {}

impl<'a, T: Object> std::fmt::Display for Delete<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let conditions = format!("{{ {} }}", self.conditions.iter().format(", "));
        let name = format!("delete_{}", T::name());
        let params = [(Some("where"), conditions)];

        construct_query(f, name, &params, &self.returning, self.affected_rows)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct DeleteByPk<'a, T: Object + Pk> {
    pk: T::Pk,
    returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object + Pk> Mutation for DeleteByPk<'a, T> {}

impl<'a, T: Object + Pk> std::fmt::Display for DeleteByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = format!("delete_{}_by_pk", T::name());
        let params = [(None, self.pk.to_string())];

        construct_query(f, name, &params, &self.returning, false)
    }
}
