use crate::util::{construct_query, Kind};
use crate::{Field, Object, Conditions};
use itertools::Itertools;
use crate::common::Pk;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Delete<'a, T: Object> {
    conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    affected_rows: bool,
    returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> ToString for Delete<'a, T> {
    fn to_string(&self) -> String {
        let conditions = format!("{{ {} }}", self.conditions.iter().format(", "));
        let name = format!("delete_{}", T::name());
        let params = [(Some("where"), conditions)];

        construct_query(
            Kind::Mutation,
            name,
            &params,
            &self.returning,
            self.affected_rows,
        )
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct DeleteByPk<'a, T: Object + Pk> {
    pk: T::Pk,
    returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object + Pk> ToString for DeleteByPk<'a, T> {
    fn to_string(&self) -> String {
        let name = format!("delete_{}_by_pk", T::name());
        let params = [(None, self.pk.to_string())];

        construct_query(Kind::Mutation, name, &params, &self.returning, false)
    }
}
