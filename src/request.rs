use crate::error::Error;
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

#[cfg(feature = "native")]
pub async fn request(
    url: &str,
    body: String,
    token: Option<String>,
    admin: Option<String>,
) -> Result<Value, Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(token) = token {
        headers.insert("authorization", token.parse().unwrap());
    }
    if let Some(admin) = admin {
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

#[cfg(all(feature = "wasm", not(feature = "native")))]
pub async fn request(
    url: &str,
    body: String,
    token: Option<String>,
    admin: Option<String>,
) -> Result<Value, Error> {
    let headers = gloo_net::http::Headers::new();
    if let Some(token) = token {
        headers.set("authorization", &token);
    }
    if let Some(admin) = admin {
        headers.set("x-hasura-admin-secret", &admin);
    }

    log::trace!("GraphQL request  {}", body);
    let now = wasm_timer::Instant::now();

    let text = gloo_net::http::Request::post(url)
        .headers(headers)
        .body(body)
        .send()
        .await?
        .text()
        .await?;

    log::trace!("response after {:?}", now.elapsed());
    log::trace!("GraphQL response {}", text);

    match serde_json::from_str(&text)? {
        Response::Data { data } => Ok(data),
        Response::Errors { errors } => Err(Error::Hasura(errors)),
    }
}
