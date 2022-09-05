// use itertools::Itertools;
//
// use crate::common::OnConflict;
// use crate::constructor::construct_query;
// use crate::{serializer, Fields, Mutation, Object};
// use serde::de::DeserializeOwned;
// use serde::Serialize;
// use std::fmt::Formatter;
//
// #[derive(derive_builder::Builder)]
// #[builder(pattern = "owned")]
// #[builder(setter(into, strip_option))]
// pub struct Insert<'a, T: Object> {
//     pub objects: Vec<T::Draft>,
//     #[builder(default)]
//     pub affected_rows: bool,
//     #[builder(default)]
//     pub on_conflict: Option<OnConflict>,
//     #[builder(default)]
//     pub returning: Fields<'a, T>,
// }
//
// impl<'a, T: Object + DeserializeOwned + Serialize> Mutation<T> for Insert<'a, T> {
//     type Out = Vec<T>;
//
//     fn name() -> String {
//         format!("insert_{}", T::name())
//     }
// }
//
// impl<'a, T: Object + Serialize + DeserializeOwned> std::fmt::Display for Insert<'a, T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let objects = self
//             .objects
//             .iter()
//             .map(|x| serializer::to_string(x, true).unwrap())
//             .format(", ");
//
//         let name = Self::name();
//         let params = [(Some("objects"), format!("[ {} ]", objects))];
//
//         let rows = self.affected_rows;
//
//         construct_query(f, &name, &params, &self.returning, rows, true)
//     }
// }
//
// #[derive(derive_builder::Builder)]
// #[builder(pattern = "owned")]
// #[builder(setter(into, strip_option))]
// pub struct InsertOne<'a, T: Object> {
//     pub object: T::Draft,
//     #[builder(default)]
//     pub on_conflict: Option<OnConflict>,
//     #[builder(default)]
//     pub returning: Fields<'a, T>,
// }
//
// impl<'a, T: Object + DeserializeOwned + Serialize> Mutation<T> for InsertOne<'a, T> {
//     type Out = Option<T>;
//
//     fn name() -> String {
//         format!("insert_{}_one", T::name())
//     }
// }
//
// impl<'a, T: Object + Serialize + DeserializeOwned> std::fmt::Display for InsertOne<'a, T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let params = [(
//             Some("object"),
//             serializer::to_string(&self.object, true).unwrap(),
//         )];
//         let name = Self::name();
//
//         construct_query(f, &name, &params, &self.returning, false, false)
//     }
// }
