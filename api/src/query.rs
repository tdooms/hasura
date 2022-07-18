use itertools::Itertools;

use crate::common::{OrderBy, Pk};
use crate::util::construct_query;
use crate::{serializer, Conditions, Field, Fields, Object, Queryable};
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use std::marker::PhantomData;

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
    #[builder(default)]
    pub returning: Fields<'a, T>,
    #[builder(default)]
    phantom: PhantomData<T>,
}

impl<'a, T: Object + DeserializeOwned> Queryable<T> for Query<'a, T> {
    type Out = Vec<T>;

    fn name() -> String {
        T::name().to_string()
    }
}

impl<'a, T: Object + DeserializeOwned> std::fmt::Display for Query<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            params.push((Some("where"), format!("{{{}}}", conditions)));
        }

        let name = Self::name();
        construct_query(f, &name, &params, &self.returning, false, false)
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
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + DeserializeOwned + Pk> Queryable<T> for QueryByPk<'a, T> {
    type Out = Option<T>;

    fn name() -> String {
        format!("{}_by_pk", T::name())
    }
}

impl<T: Object + Pk + DeserializeOwned> std::fmt::Display for QueryByPk<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = [(None, serializer::to_string(&self.pk, false).unwrap())];
        let name = Self::name();
        construct_query(f, &name, &params, &self.returning, false, false)
    }
}
