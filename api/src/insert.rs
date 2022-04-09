use itertools::Itertools;

use crate::common::OnConflict;
use crate::util::{construct_query, Kind};
use crate::{Field, Object};

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Insert<'a, T: Object> {
    pub objects: Vec<T>,
    #[builder(default)]
    pub affected_rows: bool,
    #[builder(default)]
    pub on_conflict: Option<OnConflict>,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> ToString for Insert<'a, T> {
    fn to_string(&self) -> String {
        let objects = self
            .objects
            .iter()
            .map(|x| format!("{{ {} }}", x.serialize()))
            .format(", ");

        let name = format!("insert_{}", T::name());
        let params = [(Some("objects"), format!("[ {} ]", objects))];

        let rows = self.affected_rows;
        construct_query(Kind::Mutation, name, &params, &self.returning, rows)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct InsertOne<'a, T: Object> {
    pub object: T,
    #[builder(default)]
    pub on_conflict: Option<OnConflict>,
    pub returning: Vec<Field<'a, T>>,
}

impl<'a, T: Object> ToString for InsertOne<'a, T> {
    fn to_string(&self) -> String {
        let params = [(Some("object"), format!("{{ {} }}", self.object.serialize()))];
        let name = format!("insert_{}_one", T::name());

        construct_query(Kind::Mutation, name, &params, &self.returning, false)
    }
}
