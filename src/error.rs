use crate::request::GraphqlError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "wasm")]
    #[error("Request error: {0}")]
    Request(#[from] gloo_net::Error),

    #[cfg(feature = "native")]
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Hasura error: {0:?}")]
    Hasura(Vec<GraphqlError>),

    #[error("Internal empty error")]
    Empty,
}

pub type Result<T> = std::result::Result<T, Error>;
