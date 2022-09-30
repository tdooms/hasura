use crate::{Conditions, Field, Fields, Hasura, Queryable, OrderBy, Builder, Separated, Braced};
use serde::de::DeserializeOwned;
use std::fmt::Formatter;

pub struct Query<'a, T: Hasura> {
    pub distinct_on: Option<Field<'a, T>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub conditions: Option<Conditions<'a, T>>,
    pub order_by: Vec<OrderBy<'a, T>>,
    pub returning: Fields<'a, T>,
}

impl<'a, T: Hasura> Query<'a, T> {
    pub fn new() -> Self {
        Self {
            distinct_on: None,
            limit: None,
            offset: None,
            conditions: None,
            order_by: vec![],
            returning: T::all(),
        }
    }

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
        Builder::new(Self::name(), &self.returning)
            .maybe("order_by", &Separated(self.order_by.as_ref()))
            .maybe("distinct_on", &self.distinct_on)
            .maybe("limit", &self.limit)
            .maybe("offset", &self.offset)
            .maybe("where", &Braced(&self.conditions))
            .write(f)
    }
}

// TODO: understand this
// pub struct QueryAggregate {
//
// }