use crate::error::{Error, Result};
use serde_json::Value;

#[derive(serde::Deserialize, Debug)]
pub struct GraphqlError {
    pub extensions: Value,
    pub message: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response {
    Data { data: Value },
    Errors { errors: Vec<GraphqlError> },
}

pub async fn request(url: &str, body: String, token: Option<String>) -> Result<Value> {
    let mut headers = reqwest::header::HeaderMap::new();

    if let Some(token) = token {
        headers.insert("authorization", token.parse().unwrap());
    }

    let text = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str(&text)? {
        Response::Data { data } => Ok(data),
        Response::Errors { errors } => Err(Error::Hasura(errors)),
    }
}
