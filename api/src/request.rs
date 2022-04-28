use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

// #[derive(serde::Deserialize, Debug)]
// #[serde(untagged)]
// enum Response<T> {
//     Data { data: T },
//     Errors { errors: Vec<GraphqlError> },
// }

// pub async fn request<T: Deserialize + Debug>(
//     headers: HashMap<String, String>,
// ) -> Result<T, String> {
// }
