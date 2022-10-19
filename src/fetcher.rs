use std::collections::HashMap;
use crate::{Error, Result};
use serde_json::Value;
use std::fmt::{Debug};
use std::str::FromStr;
use reqwest::header::{HeaderName, HeaderValue};

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

async fn request(url: &str, body: String, headers: HashMap<String, String>) -> Result<Value> {
    let mut temp = reqwest::header::HeaderMap::new();
    for (key, value) in headers {
        temp.insert(HeaderName::from_str(&key).unwrap(), HeaderValue::from_str(&value).unwrap());
    }

    let text = reqwest::Client::new()
        .post(url)
        .headers(temp)
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
    pub headers: HashMap<String, String>,
}

impl<O> Fetcher<O> {
    pub fn new<Fn: FnOnce(Value) -> Result<O> + 'static>(body: String, extract: Fn) -> Self {
        let inner = body.to_string().replace('"', "\\\"");
        let body = format!("{{\"query\":\"{inner}\"}}");

        Self { body, extract: Box::new(extract), headers: HashMap::new(), }
    }

    pub fn header(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn admin(mut self, admin: impl ToString) -> Self {
        self.headers.insert("x-hasura-admin-secret".to_string(), admin.to_string());
        self
    }

    pub fn token(mut self, token: impl ToString) -> Self {
        self.headers.insert("authorization".to_string(), token.to_string());
        self
    }

    pub async fn send(self, url: &str) -> Result<O> {
        let val = request(url, self.body, self.headers).await?;
        (self.extract)(val)
    }
}