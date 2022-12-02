use crate::fetcher::GraphqlError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Hasura error: {0:?}")]
    Hasura(Vec<GraphqlError>),

    #[error("Endpoint not found: code={code} path={path}")]
    NotFound{code: String, path: String},

    #[error("Internal empty error")]
    Empty,
}

pub type Result<T> = std::result::Result<T, Error>;
