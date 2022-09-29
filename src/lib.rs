pub use attributes::*;
pub use queries::*;

pub use segments::*;
pub use error::*;
pub use fetcher::Fetcher;
pub use traits::Hasura;
pub(crate) use traits::{Queryable, Mutation};
pub(crate) use utils::builder::*;
pub(crate) use utils::serializer::to_string;

pub use derive::Hasura;

pub mod relation {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, SerializeStruct, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: Serialize,
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("data", 1)?;
        state.serialize_field("data", value)?;
        state.end()
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: Deserialize<'de>,
            D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

mod attributes;
mod segments;
mod error;
mod fetcher;
mod macros;
mod traits;
mod queries;
mod utils;
