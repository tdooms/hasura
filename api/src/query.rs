use itertools::Itertools;

use crate::common::{OrderBy, Pk};
use crate::util::{construct_query, Kind};
use crate::{Conditions, Field, Object};

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Query<'a, T: Object> {
    #[builder(default)]
    pub distinct_on: Option<Field<'a, T>>,
    #[builder(default)]
    pub limit: Option<u64>,
    #[builder(default)]
    pub offset: Option<u64>,
    #[builder(default)]
    pub order_by: Vec<OrderBy<'a, T>>,
    #[builder(default)]
    pub conditions: Vec<Conditions<'a, T>>,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> ToString for Query<'a, T> {
    fn to_string(&self) -> String {
        let mut params = vec![];

        if let Some(field) = &self.distinct_on {
            params.push((Some("distinct_on"), field.to_string()));
        }
        if let Some(limit) = &self.limit {
            params.push((Some("limit"), limit.to_string()));
        }
        if let Some(offset) = &self.offset {
            params.push((Some("offset"), offset.to_string()));
        }
        if !self.order_by.is_empty() {
            let order_by = self.order_by.iter().format(", ");
            params.push((Some("order_by"), format!("{{ {} }}", order_by)));
        }
        if !self.conditions.is_empty() {
            let conditions = self.conditions.iter().format(", ");
            params.push((Some("conditions"), format!("{{{}}}", conditions)));
        }

        construct_query(Kind::Query, T::name(), &params, &self.returning, false)
    }
}

// TODO: understand this
// pub struct QueryAggregate {
//
// }

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct QueryByPk<'a, T: Object + Pk> {
    pk: T::Pk,
    returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object + Pk> ToString for QueryByPk<'a, T> {
    fn to_string(&self) -> String {
        let params = [(None, self.pk.to_string())];
        construct_query(Kind::Query, T::name(), &params, &self.returning, false)
    }
}
