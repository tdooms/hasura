use crate::common::Pk;
use crate::util::construct_query;
use crate::{Conditions, Fields, Mutation, Object};
use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Delete<'a, T: Object> {
    conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    affected_rows: bool,
    #[builder(default)]
    pub returning: Fields<'a, T>,
    #[builder(default)]
    phantom: PhantomData<T>,
}

impl<'a, T: Object + DeserializeOwned> Mutation<T> for Delete<'a, T> {
    type Out = Vec<T>;
}

impl<'a, T: Object> std::fmt::Display for Delete<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let conditions = format!("{{ {} }}", self.conditions.iter().format(", "));
        let name = format!("delete_{}", T::name());
        let params = [(Some("where"), conditions)];

        construct_query(f, &name, &params, &self.returning, self.affected_rows, true)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct DeleteByPk<'a, T: Object + Pk> {
    pk: T::Pk,
    #[builder(default)]
    pub returning: Fields<'a, T>,
    #[builder(default)]
    phantom: PhantomData<T>,
}

impl<'a, T: Object + DeserializeOwned + Pk> Mutation<T> for DeleteByPk<'a, T> {
    type Out = Option<T>;
}

impl<'a, T: Object + Pk> std::fmt::Display for DeleteByPk<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = format!("delete_{}_by_pk", T::name());
        let params = [(None, serde_json::to_string(&self.pk).unwrap())];

        construct_query(f, &name, &params, &self.returning, false, false)
    }
}
