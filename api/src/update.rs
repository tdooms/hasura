use itertools::Itertools;

use crate::common::{Conditions, Pk};
use crate::util::{construct_query, Kind};
use crate::{Field, Object};

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Update<'a, T: Object> {
    pub set: T,
    pub conditions: Vec<Conditions<'a, T>>,
    #[builder(default)]
    pub affected_rows: bool,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> ToString for Update<'a, T> {
    fn to_string(&self) -> String {
        let mut params = vec![(Some("_set"), format!("{{ {} }}", self.set.serialize()))];

        if !self.conditions.is_empty() {
            let conditions = format!("{{ {} }}", self.conditions.iter().format(", "));
            params.push((Some("where"), conditions));
        }

        let name = format!("update_{}", T::name());

        let rows = self.affected_rows;
        construct_query(Kind::Mutation, name, &params, &self.returning, rows)
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

impl<'a, T: Object + Pk> ToString for UpdateByPk<'a, T> {
    fn to_string(&self) -> String {
        let pk = format!("{{ {} }}", self.pk.to_string());
        let name = format!("update_{}_by_pk", T::name());

        let params = [
            (Some("_set"), format!("{{ {} }}", self.set.serialize())),
            (Some("pk_columns"), pk),
        ];

        construct_query(Kind::Mutation, name, &params, &self.returning, false)
    }
}
