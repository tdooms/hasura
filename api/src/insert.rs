use itertools::Itertools;

use crate::common::OnConflict;
use crate::util::construct_query;
use crate::{Fields, Mutation, Object};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Formatter;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Insert<'a, T: Object> {
    pub objects: Vec<T::Draft>,
    #[builder(default)]
    pub affected_rows: bool,
    #[builder(default)]
    pub on_conflict: Option<OnConflict>,
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + DeserializeOwned + Serialize> Mutation<T> for Insert<'a, T> {
    type Out = Vec<T>;
}

impl<'a, T: Object + Serialize> std::fmt::Display for Insert<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let objects = self
            .objects
            .iter()
            .map(|x| serde_json::to_string(x).unwrap())
            .format(", ");

        let name = format!("insert_{}", T::name());
        let params = [(Some("objects"), format!("[ {} ]", objects))];

        let rows = self.affected_rows;

        construct_query(f, &name, &params, &self.returning, rows, true)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct InsertOne<'a, T: Object> {
    pub object: T::Draft,
    #[builder(default)]
    pub on_conflict: Option<OnConflict>,
    #[builder(default)]
    pub returning: Fields<'a, T>,
}

impl<'a, T: Object + DeserializeOwned + Serialize> Mutation<T> for InsertOne<'a, T> {
    type Out = Option<T>;
}

impl<'a, T: Object + Serialize> std::fmt::Display for InsertOne<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let params = [(Some("object"), serde_json::to_string(&self.object).unwrap())];
        let name = format!("insert_{}_one", T::name());

        construct_query(f, &name, &params, &self.returning, false, false)
    }
}
