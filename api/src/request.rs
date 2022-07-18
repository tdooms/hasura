use crate::error::Error;
use gloo_net::http::{Headers, Request};
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

pub async fn request(
    url: &str,
    body: String,
    token: Option<String>,
    admin: Option<String>,
) -> Result<Value, Error> {
    let headers = Headers::new();
    if let Some(token) = token {
        headers.set("authorization", &token);
    }
    if let Some(admin) = admin {
        headers.set("x-hasura-admin-secret", &admin);
    }

    let text = Request::post(url)
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
