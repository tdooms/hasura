use crate::error::Error;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::Value;
use std::fmt::Debug;

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

pub async fn request(
    url: &str,
    body: String,
    token: Option<String>,
    admin: Option<String>,
) -> Result<Value, Error> {
    let mut headers = HeaderMap::new();
    if let Some(token) = token {
        headers.insert("authorization", token.parse().unwrap());
    }
    if let Some(admin) = admin {
        headers.insert("x-hasura-admin-secret", admin.parse().unwrap());
    }

    let text = Client::new()
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
