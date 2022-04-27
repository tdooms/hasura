use itertools::Itertools;

use crate::common::OnConflict;
use crate::util::construct_query;
use crate::{Encode, Field, Mutation, Object};
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
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object + Encode> Mutation for Insert<'a, T> {}

impl<'a, T: Object + Encode> std::fmt::Display for Insert<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let objects = self.objects.iter().map(|x| x.encode()).format(", ");

        let name = format!("insert_{}", T::name());
        let params = [(Some("objects"), format!("[ {} ]", objects))];

        let rows = self.affected_rows;

        construct_query(f, name, &params, &self.returning, rows)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct InsertOne<'a, T: Object> {
    pub object: T::Draft,
    #[builder(default)]
    pub on_conflict: Option<OnConflict>,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object + Encode> Mutation for InsertOne<'a, T> {}

impl<'a, T: Object + Encode> std::fmt::Display for InsertOne<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let params = [(Some("object"), self.object.encode())];
        let name = format!("insert_{}_one", T::name());

        construct_query(f, name, &params, &self.returning, false)
    }
}
