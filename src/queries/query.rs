use crate::{Conditions, Field, Fields, Hasura, Queryable, OrderBy, Builder, Separated};
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use itertools::Itertools;

pub struct Query<'a, T: Hasura> {
    pub distinct_on: Option<Field<'a, T>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub conditions: Option<Conditions<'a, T>>,
    pub order_by: Vec<OrderBy<'a, T>>,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> Default for Query<'a, T> {
    fn default() -> Self {
        Query {
            distinct_on: None,
            limit: None,
            offset: None,
            conditions: None,
            order_by: vec![],
            returning: T::all(),
        }
    }
}

impl<'a, T: Hasura> Query<'a, T> {
    pub fn distinct_on(mut self, distinct_on: Field<'a, T>) -> Self {
        self.distinct_on = Some(distinct_on);
        self
    }
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn conditions(mut self, conditions: Conditions<'a, T>) -> Self {
        self.conditions = Some(conditions);
        self
    }
    pub fn order_by(mut self, order_by: Vec<OrderBy<'a, T>>) -> Self {
        self.order_by = order_by;
        self
    }
    pub fn returning(mut self, returning: Fields<'a, T>) -> Self {
        self.returning = returning;
        self
    }
}

impl<'a, T: Hasura + DeserializeOwned> Queryable<T> for Query<'a, T> {
    type Out = Vec<T>;
    fn name() -> String {
        T::table().to_string()
    }
}

impl<'a, T: Hasura + DeserializeOwned> std::fmt::Display for Query<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Builder::default()
            .name(Self::name())
            .returning(&self.returning)
            .vector("order_by", &Separated(self.order_by.as_ref()))
            .maybe("distinct_on", self.distinct_on.as_ref())
            .maybe("limit", self.limit.as_ref())
            .maybe("offset", self.offset.as_ref())
            .maybe("where", self.conditions.as_ref())
            .build(f)
    }
}

// TODO: understand this
// pub struct QueryAggregate {
//
// }

// #[derive(derive_builder::Builder)]
// #[builder(pattern = "owned")]
// pub struct QueryByPk<'a, T: Object + Pk> {
//     pk: T::Pk,
//     #[builder(default)]
//     pub returning: Fields<'a, T>,
// }
//
// impl<'a, T: Object + DeserializeOwned + Pk> Queryable<T> for QueryByPk<'a, T> {
//     type Out = Option<T>;
//
//     fn name() -> String {
//         format!("{}_by_pk", T::name())
//     }
// }
//
// impl<T: Object + Pk + DeserializeOwned> std::fmt::Display for QueryByPk<'_, T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let params = [(None, serializer::to_string(&self.pk, false).unwrap())];
//         let name = Self::name();
//         construct_query(f, &name, &params, &self.returning, false, false)
//     }
// }
