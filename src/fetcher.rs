use crate::{Error, Result};
use serde_json::Value;
use std::fmt::{Debug};

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

async fn request(url: &str, body: String, t: Option<String>, a: Option<String>) -> Result<Value> {
    let mut headers = reqwest::header::HeaderMap::new();

    if let Some(token) = t {
        headers.insert("authorization", token.parse().unwrap());
    }

    if let Some(admin) = a {
        headers.insert("x-hasura-admin-secret", admin.parse().unwrap());
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

pub struct Fetcher<O> {
    pub body: String,
    pub extract: Box<dyn FnOnce(Value) -> Result<O>>,

    pub token: Option<String>,
    pub admin: Option<String>,
}

impl<O> Fetcher<O> {
    pub fn new<Fn: FnOnce(Value) -> Result<O> + 'static>(body: String, extract: Fn) -> Self {
        let body = body.to_string().replace('"', "\\\"");
        Self {
            body: format!("{{\"query\": \"{}\"}}", body),
            extract: Box::new(extract),
            token: None,
            admin: None
        }
    }

    pub fn token(mut self, token: impl Into<Option<String>>) -> Self {
        self.token = token.into();
        self
    }

    pub fn admin(mut self, admin: impl Into<Option<String>>) -> Self {
        self.admin = admin.into();
        self
    }

    pub async fn send(self, url: &str) -> Result<O> {
        let val = request(url, self.body, self.token, self.admin).await?;
        (self.extract)(val)
    }
}